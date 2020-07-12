use json;
use skeleton::srt::SRT;

#[derive(Debug)]
pub struct RegionAttachment {
    pub name: Option<String>,
    pub srt: SRT,
    // fps: Option<f32>,
    // mode: Option<String>,
    //vertices: Option<Vec<??>>     // TODO: ?
}

impl RegionAttachment {
    pub fn new(attachment: json::Attachment, name: Option<String>) -> RegionAttachment {
        let (w2, h2) = (
            attachment.width.unwrap_or(0f32) * 0.5,
            attachment.height.unwrap_or(0f32) * 0.5,
        );

        RegionAttachment {
            name: attachment.name.or(name),
            srt: SRT::new(
                attachment.scale_x.unwrap_or(1.0) * w2,
                attachment.scale_y.unwrap_or(1.0) * h2,
                attachment.rotation.unwrap_or(0.0),
                attachment.x.unwrap_or(0.0),
                attachment.y.unwrap_or(0.0),
            ),
        }
    }
}
