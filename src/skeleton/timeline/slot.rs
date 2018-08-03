use json;
use skeleton::error::SkeletonError;
use super::CurveTimelines;

pub struct SlotTimeline {
    attachment: Vec<json::SlotAttachmentTimeline>,
    color: CurveTimelines<[u8; 4]>,
}

impl SlotTimeline {
    pub fn from_json(json: json::SlotTimeline) -> Result<SlotTimeline, SkeletonError> {
        let color = CurveTimelines::from_json_vec(json.color)?;
        Ok(SlotTimeline {
            attachment: json.attachment.unwrap_or(Vec::new()),
            color
        })
    }

    pub fn interpolate_color(&self, elapsed: f32) -> [u8; 4] {
        self.color.interpolate(elapsed).unwrap_or([255, 255, 255, 255])
    }

    pub fn interpolate_attachment(&self, elapsed: f32) -> Option<Option<&str>> {
        if self.attachment.is_empty() || elapsed < self.attachment[0].time {
            None
        } else {
            let pos = self.attachment.iter().position(|a| elapsed < a.time).unwrap_or(self.attachment.len());
            Some(self.attachment[pos - 1].name.as_ref().map(|n| &**n))
        }
    }

    pub fn get_attachment_names(&self) -> Vec<&str> {
        self.attachment.iter()
            .filter_map(|t| t.name.as_ref().map(|n| &**n)).collect()
    }
}
