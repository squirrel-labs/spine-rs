use super::bone::Bone;
use super::SkeletonError;
use super::json;
use super::util;
use rustc_hex::{FromHex, FromHexError};

/// skeleton slot
pub struct Slot {
    pub name: String,
    pub bone_index: usize,
    pub color: [u8; 4],
    pub attachment: Option<String>,
}

impl Slot {
    pub fn from_json(slot: json::Slot, bones: &[Bone]) -> Result<Slot, SkeletonError> {
        let bone_index = util::bone_index(&slot.bone, &bones)?;
        let color = match slot.color {
            Some(c) => {
                let v = c.from_hex()?;
                if v.len() != 4 {
                    return Err(SkeletonError::InvalidColor(FromHexError::InvalidHexLength));
                }
                [v[0], v[1], v[2], v[3]]
            }
            None => [255, 255, 255, 255],
        };

        Ok(Slot {
            name: slot.name,
            bone_index,
            color,
            attachment: slot.attachment,
        })
    }
}
