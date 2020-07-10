use super::Interpolate;
use json;
use rustc_hex::{FromHex, FromHexError};
use skeleton::error::SkeletonError;

const BEZIER_SEGMENTS: usize = 10;

/// Curve trait to define struct with curve property (unwrapped to Linear)
pub trait Curve<T> {
    fn time(&self) -> f32;
    fn curve(&self) -> json::TimelineCurve;
    fn value(&self) -> Result<T, SkeletonError>;
}

/// Macro rule to implement curve based on json structs
/// The only non trivial property is the `value`
macro_rules! impl_curve {
    ($to:ty, $from:ty, $f:expr) => {
        impl Curve<$from> for $to {
            fn time(&self) -> f32 {
                self.time
            }
            fn curve(&self) -> json::TimelineCurve {
                self.curve
                    .clone()
                    .unwrap_or(json::TimelineCurve::CurveLinear)
            }
            fn value(&self) -> Result<$from, SkeletonError> {
                $f(&self)
            }
        }
    };
}

impl_curve!(
    json::BoneTranslateTimeline,
    (f32, f32),
    |t: &json::BoneTranslateTimeline| { Ok((t.x.unwrap_or(0f32), t.y.unwrap_or(0f32))) }
);

impl_curve!(
    json::BoneScaleTimeline,
    (f32, f32),
    |t: &json::BoneScaleTimeline| { Ok((t.x.unwrap_or(1f32), t.y.unwrap_or(1f32))) }
);

impl_curve!(
    json::BoneRotateTimeline,
    f32,
    |t: &json::BoneRotateTimeline| {
        let mut angle = t.angle.unwrap_or(0f32);
        while angle > 180.0 {
            angle -= 360.0;
        }
        while angle < -180.0 {
            angle += 360.0;
        }
        Ok(angle)
    }
);

impl_curve!(
    json::SlotColorTimeline,
    [u8; 4],
    |t: &json::SlotColorTimeline| {
        Ok(match t.color {
            Some(ref c) => {
                let v = c.from_hex()?;
                if v.len() != 4 {
                    return Err(SkeletonError::InvalidColor(FromHexError::InvalidHexLength));
                }
                [v[0], v[1], v[2], v[3]]
            }
            None => [255, 255, 255, 255],
        })
    }
);

impl Curve<Option<String>> for json::SlotAttachmentTimeline {
    fn time(&self) -> f32 {
        self.time
    }
    fn curve(&self) -> json::TimelineCurve {
        json::TimelineCurve::CurveStepped
    }
    fn value(&self) -> Result<Option<String>, SkeletonError> {
        Ok(self.name.clone())
    }
}

pub struct CurveTimeline<T> {
    pub time: f32,
    pub curve: json::TimelineCurve,
    pub points: Option<(Vec<f32>, Vec<f32>)>, // bezier curve interpolations points
    pub value: T,
}

impl<T> CurveTimeline<T> {
    /// interpolation values (x, y)
    /// Sets the control handle positions for an interpolation bezier curve used to transition
    /// from this keyframe to the next.
    /// cx1 and cx2 are from 0 to 1, representing the percent of time between the two keyframes.
    /// cy1 and cy2 are the percent of the difference between the keyframe's values.
    pub fn compute_points(curve: &json::TimelineCurve) -> Option<(Vec<f32>, Vec<f32>)> {
        let (cx1, cy1, cx2, cy2) = match *curve {
            json::TimelineCurve::CurveStepped | json::TimelineCurve::CurveLinear => return None, // no interpolation: early return
            json::TimelineCurve::CurveBezier(ref p) => (p[0], p[1], p[2], p[3]),
        };

        let subdiv1 = 1f32 / BEZIER_SEGMENTS as f32;
        let subdiv2 = subdiv1 * subdiv1;
        let subdiv3 = subdiv2 * subdiv1;
        let (pre1, pre2, pre4, pre5) = (
            3f32 * subdiv1,
            3f32 * subdiv2,
            6f32 * subdiv2,
            6f32 * subdiv3,
        );
        let (tmp1x, tmp1y) = (-cx1 * 2f32 + cx2, -cy1 * 2f32 + cy2);
        let (tmp2x, tmp2y) = ((cx1 - cx2) * 3f32 + 1f32, (cy1 - cy2) * 3f32 + 1f32);
        let mut dfx = cx1 * pre1 + tmp1x * pre2 + tmp2x * subdiv3;
        let mut dfy = cy1 * pre1 + tmp1y * pre2 + tmp2y * subdiv3;
        let (mut ddfx, mut ddfy) = (tmp1x * pre4 + tmp2x * pre5, tmp1y * pre4 + tmp2y * pre5);
        let (dddfx, dddfy) = (tmp2x * pre5, tmp2y * pre5);

        let (mut vec_x, mut vec_y) = (
            Vec::with_capacity(BEZIER_SEGMENTS),
            Vec::with_capacity(BEZIER_SEGMENTS),
        );
        let (mut x, mut y) = (dfx, dfy);
        for _ in 0..BEZIER_SEGMENTS {
            vec_x.push(x);
            vec_y.push(y);
            dfx += ddfx;
            dfy += ddfy;
            ddfx += dddfx;
            ddfy += dddfy;
            x += dfx;
            y += dfy;
        }
        Some((vec_x, vec_y))
    }

    /// Get percent conversion depending on curve type
    pub fn get_percent(&self, percent: f32) -> f32 {
        let &(ref x, ref y) = match self.curve {
            json::TimelineCurve::CurveStepped => return 0f32,
            json::TimelineCurve::CurveLinear => return percent,
            json::TimelineCurve::CurveBezier(..) => self.points.as_ref().unwrap(),
        };

        // bezier curve
        match x.iter().position(|&xi| percent < xi) {
            Some(0) => y[0] * percent / x[0],
            Some(i) => y[i] + (y[i] - y[i - 1]) * (percent - x[i - 1]) / (x[i] - x[i - 1]),
            None => {
                let (x, y) = (x[BEZIER_SEGMENTS - 1], y[BEZIER_SEGMENTS - 1]);
                y + (1f32 - y) * (percent - x) / (1f32 - x)
            }
        }
    }
}

/// Set of timelines
pub struct CurveTimelines<T> {
    pub timelines: Vec<CurveTimeline<T>>,
}

impl<T: Interpolate + Clone> CurveTimelines<T> {
    /// Converts vector of json timelines to vector or timelines
    pub fn from_json_vec<U: Curve<T>>(
        jtimelines: Option<Vec<U>>,
    ) -> Result<CurveTimelines<T>, SkeletonError> {
        match jtimelines {
            None => Ok(CurveTimelines {
                timelines: Vec::new(),
            }),
            Some(timelines) => {
                let mut curves = Vec::with_capacity(timelines.len());
                for t in timelines.into_iter() {
                    let value = t.value()?;
                    let curve = t.curve();
                    let points = CurveTimeline::<T>::compute_points(&curve);
                    curves.push(CurveTimeline {
                        time: t.time(),
                        curve,
                        value,
                        points,
                    });
                }
                Ok(CurveTimelines { timelines: curves })
            }
        }
    }

    /// interpolates `value` in the interval containing elapsed
    pub fn interpolate(&self, elapsed: f32) -> Option<T> {
        if self.timelines.is_empty() || elapsed < self.timelines[0].time {
            return None;
        }

        if let Some(w) = self.timelines.windows(2).find(|&w| elapsed < w[1].time) {
            let percent = (elapsed - w[0].time) / (w[1].time - w[0].time);
            let curve_percent = w[0].get_percent(percent);
            Some(w[0].value.interpolate(&w[1].value, curve_percent))
        } else {
            Some(self.timelines[self.timelines.len() - 1].value.clone())
        }
    }
}
