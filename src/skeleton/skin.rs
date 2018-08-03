use std::collections::HashMap;
use super::attachment::Attachment;

/// Skin
/// defines a set of slot with custom attachments
/// slots: Vec<(slot_index, HashMap<custom_attachment_name, Attachment>)>
/// TODO: simpler architecture
pub struct Skin {
    /// all slots modified by the skin, the default skin contains all skeleton bones
    pub slots: Vec<(usize, HashMap<String, Attachment>)>,
}

impl Skin {
    /// find attachment in a skin
    pub fn find(&self, slot_index: usize, attach_name: &str) -> Option<&Attachment> {
        self.slots
            .iter()
            .filter_map(|&(i, ref attachs)| {
                if i == slot_index {
                    attachs.get(attach_name)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn attachments(&self) -> Vec<&Attachment> {
        self.slots
            .iter()
            .flat_map(|(_, attachs)| attachs.iter().map(|(_, attach)| attach))
            .collect()
    }

    /// get all attachments and their positions to setup the skeleton's skin
    pub fn attachment_positions(&self) -> Vec<(&str, Option<&[[f32; 2]; 4]>)> {
        self.slots
            .iter()
            .flat_map(|&(_, ref attachs)| {
                attachs.iter().map(|(name, ref attach)| match attach {
                    &Attachment::Region(region) => (&**name, Some(&region.positions)),
                    &Attachment::Mesh(_) => (&**name, None),
                })
            })
            .collect()
    }
}
