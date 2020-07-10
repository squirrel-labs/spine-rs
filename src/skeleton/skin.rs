use super::attachment::Attachment;
use std::collections::HashMap;

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
}
