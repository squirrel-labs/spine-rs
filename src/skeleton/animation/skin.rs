use super::iter::AnimationIter;
use super::sprite::Sprites;
use super::AttachmentWrapper;
use skeleton::timeline::{BoneTimeline, SlotTimeline};
use skeleton::{
    animation::Animation, bone::Bone, error::SkeletonError, slot::Slot, srt::SRT, Skeleton,
};

/// Struct to handle animated skin and calculate sprites
pub struct SkinAnimation<'a> {
    anim_bones: Vec<(&'a Bone, Option<&'a BoneTimeline>)>,
    anim_slots: Vec<(&'a Slot, AttachmentWrapper<'a>, Option<&'a SlotTimeline>)>,
    duration: f32,
    transition_anim: Option<(
        f32,
        Vec<(&'a Bone, Option<BoneTimeline>)>,
        Vec<(&'a Slot, Option<SlotTimeline>)>,
    )>,
}

impl<'a> SkinAnimation<'a> {
    /// Iterator<Item=Vec<CalculatedSlot>> where item are modified with timelines
    pub fn new(
        skeleton: &'a Skeleton,
        skin: &str,
        animation: Option<&str>,
        transition: Option<Animation>,
    ) -> Result<SkinAnimation<'a>, SkeletonError> {
        // search all attachments defined by the skin name (use 'default' skin if not found)
        let skin = skeleton.get_skin(skin)?;
        let default_skin = skeleton.get_skin("default")?;

        // get animation
        let (animation, duration) = if let Some(animation) = animation {
            let anim = skeleton
                .animations
                .get(animation)
                .ok_or_else(|| SkeletonError::AnimationNotFound(animation.to_owned()))?;
            (Some(anim), anim.duration)
        } else {
            (None, 0f32)
        };

        // get bone related data
        let anim_bones = skeleton
            .bones
            .iter()
            .enumerate()
            .map(|(i, b)| {
                (
                    b,
                    animation.and_then(|anim| {
                        anim.bones
                            .iter()
                            .find(|&&(idx, _)| idx == i)
                            .map(|&(_, ref a)| a)
                    }),
                )
            })
            .collect();

        let find_attach =
            |i: usize, name: &str| skin.find(i, name).or_else(|| default_skin.find(i, name));

        let transition_anim = transition.and_then(|mut trans| {
            Some((
                trans.duration,
                skeleton
                    .bones
                    .iter()
                    .enumerate()
                    .map(|(i, b)| {
                        (b, {
                            let j = trans.bones.iter().position(|&(idx, _)| idx == i);
                            j.map(|i| trans.bones.swap_remove(i)).map(|(_, anim)| anim)
                        })
                    })
                    .collect(),
                skeleton
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(i, s)| {
                        let j = trans.slots.iter().position(|&(idx, _)| idx == i);
                        let anim = j.map(|i| trans.slots.swap_remove(i)).map(|(_, anim)| anim);
                        (s, anim)
                    })
                    .collect(),
            ))
        });

        // get slot related data
        let anim_slots = skeleton
            .slots
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let anim = animation.and_then(|anim| {
                    anim.slots
                        .iter()
                        .find(|&&(idx, _)| idx == i)
                        .map(|&(_, ref anim)| anim)
                });

                let slot_attach = s.attachment.as_ref().and_then(|name| find_attach(i, &name));
                let attach = match anim.map(|anim| anim.get_attachment_names()) {
                    Some(names) => {
                        if names.is_empty() {
                            AttachmentWrapper::Static(slot_attach)
                        } else {
                            let attachments = names
                                .iter()
                                .map(|&name| (name, find_attach(i, name)))
                                .collect();
                            AttachmentWrapper::Dynamic(slot_attach, attachments)
                        }
                    }
                    None => AttachmentWrapper::Static(slot_attach),
                };
                (s, attach, anim)
            })
            .collect();

        Ok(SkinAnimation {
            duration,
            anim_bones,
            anim_slots,
            transition_anim,
        })
    }

    /// Gets duration of the longest timeline in the animation
    pub fn get_duration(&self) -> f32 {
        self.duration
    }

    /// gets all bones srts at given time
    fn get_bones_srts(&self, time: f32) -> Vec<SRT> {
        if let Some(trans) = self.transition_anim.as_ref() {
            if trans.0 < time {
                let anim = &trans.1;
                let mut srts: Vec<SRT> = Vec::with_capacity(anim.len());
                for (bone, anim) in anim {
                    srts.push(self.get_bone_srt(srts.as_ref(), bone, anim.as_ref(), time));
                }
            }
        }
        let mut srts: Vec<SRT> = Vec::with_capacity(self.anim_bones.len());
        for &(bone, anim) in &self.anim_bones {
            srts.push(self.get_bone_srt(srts.as_ref(), bone, anim, time));
        }
        srts
    }

    fn get_bone_srt(
        &self,
        srts: &[SRT],
        bone: &Bone,
        anim: Option<&BoneTimeline>,
        time: f32,
    ) -> SRT {
        let mut srt = bone.srt.clone();
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
        if let Some(ref parent_srt) = bone.parent_index.and_then(|p| srts.get(p)) {
            srt.position = parent_srt.transform(srt.position);
            if bone.inherit_rotation {
                rotation += parent_srt.rotation;
            }
            if bone.inherit_scale {
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
        srt
    }

    /// Interpolates animated slots at given time
    pub fn interpolate<'b: 'a>(&'b self, time: f32) -> Option<Sprites<'b>> {
        if time > self.duration {
            return None;
        }

        let srts = self.get_bones_srts(time);
        let iter = self.anim_slots.iter();
        Some(Sprites { iter, srts, time })
    }

    /// Creates an iterator which iterates sprites at delta seconds interval
    pub fn run<'b: 'a>(&'b self, delta: f32) -> AnimationIter<'b> {
        AnimationIter {
            skin_animation: &self,
            time: 0f32,
            delta,
        }
    }
}
