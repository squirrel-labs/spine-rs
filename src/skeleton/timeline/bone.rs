use super::CurveTimelines;
use json;
use skeleton::{error::SkeletonError, srt::SRT};

pub struct BoneTimeline {
    translate: CurveTimelines<(f32, f32)>,
    rotate: CurveTimelines<f32>,
    scale: CurveTimelines<(f32, f32)>,
}

fn arr_tup(arr: [f32; 2]) -> (f32, f32) {
    (arr[0], arr[1])
}

impl BoneTimeline {
    /// converts json data into BoneTimeline
    pub fn from_json(json: json::BoneTimeline) -> Result<BoneTimeline, SkeletonError> {
        let translate = CurveTimelines::from_json_vec(json.translate)?;
        let rotate = CurveTimelines::from_json_vec(json.rotate)?;
        let scale = CurveTimelines::from_json_vec(json.scale)?;
        Ok(BoneTimeline {
            translate,
            rotate,
            scale,
        })
    }

    pub fn from_srts(a: SRT, b: SRT, time: f32) -> BoneTimeline {
        let scale =
            CurveTimelines::<(f32, f32)>::from_srts(arr_tup(a.scale), arr_tup(b.scale), time);
        let rotate = CurveTimelines::<f32>::from_srts(a.rotation, b.rotation, time);
        let translate =
            CurveTimelines::<(f32, f32)>::from_srts(arr_tup(a.position), arr_tup(b.position), time);

        BoneTimeline {
            translate,
            rotate,
            scale,
        }
    }

    /// evaluates the interpolations for elapsed time on all timelines and
    /// returns the corresponding srt
    pub fn srt(&self, elapsed: f32) -> SRT {
        let (x, y) = self.translate.interpolate(elapsed).unwrap_or((0f32, 0f32));
        let rotation = self.rotate.interpolate(elapsed).unwrap_or(0f32);
        let (scale_x, scale_y) = self.scale.interpolate(elapsed).unwrap_or((1.0, 1.0));

        SRT::new(scale_x, scale_y, rotation, x, y)
    }
}
