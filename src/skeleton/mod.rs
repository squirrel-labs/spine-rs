pub mod animation;
pub mod attachment;
pub mod bone;
pub mod error;
pub mod skin;
pub mod slot;
pub mod srt;
pub mod timeline;
pub mod util;

use json;
use serde_json;
use std::collections::HashMap;
use std::io::Read;

// Reexport skeleton modules
use self::animation::skin::SkinAnimation;
use self::animation::Animation;
use self::attachment::Attachment;
use self::bone::Bone;
use self::error::SkeletonError;
use self::skin::Skin;
use self::slot::Slot;

/// Skeleton data converted from json and loaded into memory
pub struct Skeleton {
    /// bones for the skeleton, hierarchically ordered
    bones: Vec<Bone>,
    /// slots
    slots: Vec<Slot>,
    /// skins : key: skin name, value: slots attachments
    skins: HashMap<String, Skin>,
    /// all the animations
    animations: HashMap<String, Animation>,
}

impl Skeleton {
    /// Consumes reader (with json data) and returns a skeleton wrapping
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Skeleton, SkeletonError> {
        // read and convert as json
        let document: json::Document = serde_json::from_reader(&mut reader)?;

        // convert to skeleton (consumes document)
        Skeleton::from_json(document)
    }

    /// Creates a from_json skeleton
    /// Consumes json::Document
    fn from_json(doc: json::Document) -> Result<Skeleton, SkeletonError> {
        let mut bones = Vec::new();
        if let Some(jbones) = doc.bones {
            for b in jbones.into_iter() {
                let bone = Bone::from_json(b, &bones)?;
                bones.push(bone);
            }
        }

        let mut slots = Vec::new();
        if let Some(jslots) = doc.slots {
            for s in jslots.into_iter() {
                let slot = Slot::from_json(s, &bones)?;
                slots.push(slot);
            }
        }

        let mut animations = HashMap::new();
        for janimations in doc.animations.into_iter() {
            for (name, animation) in janimations.into_iter() {
                let animation = Animation::from_json(animation, &bones, &slots)?;
                animations.insert(name, animation);
            }
        }

        let mut skins = HashMap::new();
        for jskin in doc.skins.into_iter() {
            for (name, jslots) in jskin.into_iter() {
                let mut skin = Vec::new();
                for (name, attachments) in jslots.into_iter() {
                    let slot_index = util::slot_index(&name, &slots)?;
                    let attachments = attachments
                        .into_iter()
                        .map(|(name, attachment)| {
                            (
                                name.clone(),
                                Attachment::from_json(attachment, Some(name)).unwrap(),
                            )
                        })
                        .collect();
                    skin.push((slot_index, attachments));
                }
                skins.insert(name, Skin { slots: skin });
            }
        }

        Ok(Skeleton {
            bones,
            slots,
            skins,
            animations,
        })
    }

    /// get skin
    pub fn get_skin<'a>(&'a self, name: &str) -> Result<&'a Skin, SkeletonError> {
        self.skins
            .get(name)
            .ok_or_else(|| SkeletonError::SkinNotFound(name.to_owned()))
    }

    /// Gets a SkinAnimation which can interpolate slots at a given time
    pub fn get_animated_skin<'a>(
        &'a self,
        skin: &str,
        animation: Option<&str>,
    ) -> Result<SkinAnimation<'a>, SkeletonError> {
        SkinAnimation::new(self, skin, animation, None)
    }

    /// Gets a SkinAnimation which can interpolate slots at a given time starting from the current
    /// position
    pub fn get_animated_skin_with_transiton<'a>(
        skeleton: &'a Self,
        skin: &str,
        next_animation: &str,
        current_animation: &str,
        current_time: f32,
        start_offset: f32,
        fade_duration: f32,
    ) -> Result<SkinAnimation<'a>, SkeletonError> {
        let trans = Animation::from_animations(
            skeleton
                .animations
                .get(current_animation)
                .ok_or(SkeletonError::AnimationNotFound(
                    current_animation.to_owned(),
                ))?
                .bones
                .as_slice(),
            skeleton
                .animations
                .get(next_animation)
                .ok_or(SkeletonError::AnimationNotFound(next_animation.to_owned()))?,
            &skeleton.bones,
            current_time,
            start_offset,
            fade_duration,
        );
        SkinAnimation::new(skeleton, skin, Some(next_animation), Some(trans))
    }

    /// Returns the list of all skins names in this document.
    pub fn get_skins_names(&self) -> Vec<&str> {
        self.skins.keys().map(|k| &**k).collect()
    }

    /// Returns the list of all animations names in this document.
    pub fn get_animations_names(&self) -> Vec<&str> {
        self.animations.keys().map(|k| &**k).collect()
    }

    /// Returns the list of all attachment names in all skins in this document.
    ///
    /// The purpose of this function is to allow you to preload what you need.
    pub fn get_attachments_names(&self) -> Vec<&str> {
        let mut names: Vec<_> = self
            .skins
            .values()
            .flat_map(|skin| {
                skin.slots.iter().flat_map(|&(_, ref attach)| {
                    attach
                        .iter()
                        .map(|(k, v)| v.name().map(|n| &**n).unwrap_or(&*k))
                })
            })
            .collect();

        names.sort();
        names.dedup();
        names
    }
}
