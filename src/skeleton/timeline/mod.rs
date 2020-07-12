pub mod bone;
pub mod curve;
pub mod slot;

pub use self::bone::BoneTimeline;
pub use self::curve::{CurveTimeline, CurveTimelines};
pub use self::slot::SlotTimeline;

pub trait Interpolate {
    fn interpolate(&self, next: &Self, percent: f32) -> Self;
}

impl Interpolate for f32 {
    fn interpolate(&self, next: &Self, percent: f32) -> Self {
        *self + percent * (*next - *self)
    }
}

impl Interpolate for (f32, f32) {
    fn interpolate(&self, next: &Self, percent: f32) -> Self {
        (
            self.0 + percent * (next.0 - self.0),
            self.1 + percent * (next.1 - self.1),
        )
    }
}

impl Interpolate for [u8; 4] {
    fn interpolate(&self, next: &Self, percent: f32) -> Self {
        [
            (self[0] as f32).interpolate(&(next[0] as f32), percent) as u8,
            (self[1] as f32).interpolate(&(next[1] as f32), percent) as u8,
            (self[2] as f32).interpolate(&(next[2] as f32), percent) as u8,
            (self[3] as f32).interpolate(&(next[3] as f32), percent) as u8,
        ]
    }
}
