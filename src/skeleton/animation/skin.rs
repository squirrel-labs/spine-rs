use super::sprite::Sprites;
use skeleton::{Skeleton, error::SkeletonError, bone::Bone, slot::Slot, srt::SRT};
use skeleton::timeline::{BoneTimeline, SlotTimeline};
use super::AttachmentWrapper;
use super::iter::AnimationIter;

/// Struct to handle animated skin and calculate sprites
pub struct SkinAnimation<'a> {
    anim_bones: Vec<(&'a Bone, Option<&'a BoneTimeline>)>,
    anim_slots: Vec<(&'a Slot, AttachmentWrapper<'a>, Option<&'a SlotTimeline>)>,
    duration: f32
}

impl<'a> SkinAnimation<'a> {
    /// Iterator<Item=Vec<CalculatedSlot>> where item are modified with timelines
    pub fn new(skeleton: &'a Skeleton, skin: &str, animation: Option<&str>) -> Result<SkinAnimation<'a>, SkeletonError> {
        // search all attachments defined by the skin name (use 'default' skin if not found)
        let skin = skeleton.get_skin(skin)?;
        let default_skin = skeleton.get_skin("default")?;

        // get animation
        let (animation, duration) = if let Some(animation) = animation {
            let anim = skeleton.animations
                .get(animation)
                .ok_or_else(|| SkeletonError::AnimationNotFound(animation.to_owned()))?;
            (Some(anim), anim.duration)
        } else {
            (None, 0f32)
        };

        // get bone related data
        let anim_bones = skeleton.bones.iter().enumerate().map(|(i, b)|
            (b, animation.and_then(|anim| anim.bones.iter()
                .find(|&&(idx, _)| idx == i).map(|&(_, ref a)| a)))).collect();

        let find_attach = |i: usize, name: &str| skin.find(i, name).or_else(|| default_skin.find(i, name));

        // get slot related data
        let anim_slots = skeleton.slots.iter().enumerate().map(|(i, s)| {

            let anim = animation.and_then(|anim|
                anim.slots.iter().find(|&&(idx, _)| idx == i ).map(|&(_, ref anim)| anim));

            let slot_attach = s.attachment.as_ref().and_then(|name| find_attach(i, &name));
            let attach = match anim.map(|anim| anim.get_attachment_names()) {
                Some(names) => {
                    if names.is_empty() {
                        AttachmentWrapper::Static(slot_attach)
                    } else {
                        let attachments = names.iter().map(|&name|(name, find_attach(i, name))).collect();
                        AttachmentWrapper::Dynamic(slot_attach, attachments)
                    }
                },
                None => AttachmentWrapper::Static(slot_attach)
            };
            (s, attach, anim)
        }).collect();

        Ok(SkinAnimation {
            duration,
            anim_bones,
            anim_slots,
        })
    }

    /// Gets duration of the longest timeline in the animation
    pub fn get_duration(&self) -> f32 {
        self.duration
    }

    /// gets all bones srts at given time
    fn get_bones_srts(&self, time: f32) -> Vec<SRT> {

        let mut srts: Vec<SRT> = Vec::with_capacity(self.anim_bones.len());
        for &(b, anim) in &self.anim_bones {

            // starts with setup pose
            let mut srt = b.srt.clone();
            let mut rotation = 0.0;

            // add animation srt
            if let Some(anim_srt) = anim.map(|anim| anim.srt(time)) {
                srt.position[0] += anim_srt.position[0];
                srt.position[1] += anim_srt.position[1];
                rotation += anim_srt.rotation;
                srt.scale[0] *= anim_srt.scale[0];
                srt.scale[1] *= anim_srt.scale[1];
            }

            // inherit world from parent srt
            if let Some(ref parent_srt) = b.parent_index.and_then(|p| srts.get(p)) {
                srt.position = parent_srt.transform(srt.position);
                if b.inherit_rotation {
                    rotation += parent_srt.rotation;
                }
                if b.inherit_scale {
                    srt.scale[0] *= parent_srt.scale[0];
                    srt.scale[1] *= parent_srt.scale[1];
                }
            }

            // re-calculate sin/cos only if rotation has changed
            if rotation != 0.0 {
                srt.rotation += rotation;
                srt.cos = srt.rotation.cos();
                srt.sin = srt.rotation.sin();
            }
            srts.push(srt)
        }
        srts
    }

    /// Interpolates animated slots at given time
    pub fn interpolate<'b: 'a>(&'b self, time: f32) -> Option<Sprites<'b>> {

        if time > self.duration {
            return None;
        }

        let srts = self.get_bones_srts(time);
        let iter = self.anim_slots.iter();
        Some(Sprites {
            iter,
            srts,
            time
        })
    }

    /// Creates an iterator which iterates sprites at delta seconds interval
    pub fn run<'b: 'a>(&'b self, delta: f32) -> AnimationIter<'b> {
        AnimationIter {
            skin_animation: &self,
            time: 0f32,
            delta
        }
    }
}
