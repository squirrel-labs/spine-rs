use std::slice::Iter;
use skeleton::{srt::SRT, slot::Slot, timeline::SlotTimeline};
use super::AttachmentWrapper;

/// Interpolated slot with attachment and color
#[derive(Debug)]
pub struct Sprite<'a> {
    /// attachment name
    pub attachment: &'a str,
    /// color
    pub color: [u8; 4],
    /// srt
    pub srt: SRT
}

/// Iterator over all sprites interpolated at a given time
pub struct Sprites<'a> {
    pub iter: Iter<'a, (&'a Slot, AttachmentWrapper<'a>, Option<&'a SlotTimeline>)>,
    pub srts: Vec<SRT>,
    pub time: f32
}

impl<'a> Iterator for Sprites<'a> {
    type Item = Sprite<'a>;
    fn next<'b>(&'b mut self) -> Option<Sprite<'a>> {

        while let Some(&(slot, ref skin_attach, anim)) = self.iter.next() {

            // search animated attachment
            let (name, skin_attach) = match *skin_attach {
                AttachmentWrapper::Static(ref attach) => (None, attach),
                AttachmentWrapper::Dynamic(ref attach, ref names) => {
                    match anim.unwrap().interpolate_attachment(self.time) {
                        Some(Some(name)) => {
                            let attach = names.get(&*name).unwrap();
                            (Some(name), attach)
                        },
                        Some(None) => (None, attach),
                        None => (None, attach),
                    }
                }
            };

            // nothing to show if there is no attachment
            if let Some(ref skin_attach) = *skin_attach {

                // color
                let color = anim.map(|anim| anim.interpolate_color(self.time))
                    .unwrap_or(slot.color.clone());

                // attachment name
                let attach_name = name.or(skin_attach.name().or(slot.attachment.as_ref())
                    .map(|n| &**n))
                    .expect("no attachment name provided");

                return Some(Sprite {
                    attachment: attach_name,
                    srt: self.srts[slot.bone_index].clone(),
                    color
                })
            }
        }

        // end of iter
        None
    }
}
