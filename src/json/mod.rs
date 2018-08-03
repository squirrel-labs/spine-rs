mod attachment;
mod timeline_curve;

use std::collections::HashMap;
pub use self::timeline_curve::TimelineCurve;
pub use self::attachment::{Attachment, AttachmentType};

#[derive(Debug, Clone, Deserialize)]
pub struct Document {
    pub bones: Option<Vec<Bone>>,
    pub slots: Option<Vec<Slot>>,
    pub skins: Option<HashMap<String, HashMap<String, HashMap<String, Attachment>>>>,
    pub animations: Option<HashMap<String, Animation>>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bone {
    pub name: String,
    pub parent: Option<String>,
    pub length: Option<f32>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
    pub rotation: Option<f32>,
    pub inherit_scale: Option<bool>,
    pub inherit_rotation: Option<bool>,
    pub transform: Option<String>
}

#[derive(Debug, Clone, Deserialize)]
pub struct Slot {
    pub name: String,
    pub bone: String,
    pub color: Option<String>,
    pub attachment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Event {
    pub name: String,
    pub int: Option<i32>,
    pub float: Option<f32>,
    pub string: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Animation {
    pub bones: Option<HashMap<String, BoneTimeline>>,
    pub slots: Option<HashMap<String, SlotTimeline>>,
    pub events: Option<Vec<EventKeyframe>>,
    pub draworder: Option<Vec<DrawOrderTimeline>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BoneTimeline {
    pub translate: Option<Vec<BoneTranslateTimeline>>,
    pub rotate: Option<Vec<BoneRotateTimeline>>,
    pub scale: Option<Vec<BoneScaleTimeline>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BoneTranslateTimeline {
    pub time: f32,
    pub curve: Option<TimelineCurve>,
    pub x: Option<f32>,
    pub y: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BoneRotateTimeline {
    pub time: f32,
    pub curve: Option<TimelineCurve>,
    pub angle: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BoneScaleTimeline {
    pub time: f32,
    pub curve: Option<TimelineCurve>,
    pub x: Option<f32>,
    pub y: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SlotTimeline {
    pub attachment: Option<Vec<SlotAttachmentTimeline>>,
    pub color: Option<Vec<SlotColorTimeline>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SlotAttachmentTimeline {
    pub time: f32,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SlotColorTimeline {
    pub time: f32,
    pub color: Option<String>,
    pub curve: Option<TimelineCurve>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventKeyframe {
    pub time: f32,
    name: String,
    int: Option<i32>,
    float: Option<f32>,
    string: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DrawOrderTimeline {
    pub time: f32,
    offsets: Option<Vec<DrawOrderTimelineOffset>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DrawOrderTimelineOffset {
    slot: String,
    offset: i32,
}
