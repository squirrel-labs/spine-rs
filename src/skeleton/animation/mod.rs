//! Module to interpolate animated sprites

pub mod iter;
pub mod skin;
pub mod sprite;

use json;
use super::util;
use super::Bone;
use super::Slot;
use super::SkeletonError;
use super::timeline::{BoneTimeline, SlotTimeline};
use std::collections::HashMap;
use skeleton::attachment::Attachment;

/// Wrapper on attachment depending whether slot attachment is animated or not
pub enum AttachmentWrapper<'a> {
    Static(Option<&'a Attachment>),
    Dynamic(Option<&'a Attachment>, HashMap<&'a str, Option<&'a Attachment>>),
}

/// Animation with precomputed data
pub struct Animation {
    bones: Vec<(usize, BoneTimeline)>,
    slots: Vec<(usize, SlotTimeline)>,
    events: Vec<json::EventKeyframe>,
    draworder: Vec<json::DrawOrderTimeline>,
    duration: f32,
}

impl Animation {
    /// Creates a from_json Animation
    pub fn from_json(
        animation: json::Animation,
        bones: &[Bone],
        slots: &[Slot],
    ) -> Result<Animation, SkeletonError> {
        let duration = Animation::duration(&animation);

        let mut abones = Vec::new();
        for jbones in animation.bones.into_iter() {
            for (name, timelines) in jbones.into_iter() {
                let index = util::bone_index(&name, bones)?;
                let timeline = BoneTimeline::from_json(timelines)?;
                abones.push((index, timeline));
            }
        }

        let mut aslots = Vec::new();
        for jslots in animation.slots.into_iter() {
            for (name, timelines) in jslots.into_iter() {
                let index = util::slot_index(&name, slots)?;
                let timeline = SlotTimeline::from_json(timelines)?;
                aslots.push((index, timeline));
            }
        }

        Ok(Animation {
            duration,
            bones: abones,
            slots: aslots,
            events: animation.events.unwrap_or(Vec::new()),
            draworder: animation.draworder.unwrap_or(Vec::new()),
        })
    }

    pub fn duration(animation: &json::Animation) -> f32 {
        animation
            .bones
            .iter()
            .flat_map(|bones| {
                bones.values().flat_map(|timelines| {
                    timelines
                        .translate
                        .iter()
                        .flat_map(|translate| translate.iter().map(|e| e.time))
                        .chain(
                            timelines
                                .rotate
                                .iter()
                                .flat_map(|rotate| rotate.iter().map(|e| e.time)),
                        )
                        .chain(
                            timelines
                                .scale
                                .iter()
                                .flat_map(|scale| scale.iter().map(|e| e.time)),
                        )
                })
            })
            .chain(animation.slots.iter().flat_map(|slots| {
                slots.values().flat_map(|timelines| {
                    timelines
                        .attachment
                        .iter()
                        .flat_map(|attachment| attachment.iter().map(|e| e.time))
                        .chain(
                            timelines
                                .color
                                .iter()
                                .flat_map(|color| color.iter().map(|e| e.time)),
                        )
                })
            }))
            .fold(0.0f32, f32::max)
    }
}
