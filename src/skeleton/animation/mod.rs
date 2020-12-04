//! Module to interpolate animated sprites

pub mod iter;
pub mod skin;
pub mod sprite;

use super::timeline::{BoneTimeline, SlotTimeline};
use super::util;
use super::Bone;
use super::SkeletonError;
use super::Slot;
use json;
use skeleton::attachment::Attachment;
use std::collections::HashMap;

/// Wrapper on attachment depending whether slot attachment is animated or not
#[derive(Debug)]
pub enum AttachmentWrapper<'a> {
    Static(Option<&'a Attachment>),
    Dynamic(
        Option<&'a Attachment>,
        HashMap<&'a str, Option<&'a Attachment>>,
    ),
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

    pub fn from_animations(
        first_animation: &Animation,
        second_animation: &Animation,
        current_time: f32,
        start_offest: f32,
        duration: f32,
    ) -> Animation {
        let bones = first_animation
            .bones
            .iter()
            .map(|(i, s)| {
                if let Some((_, st)) = second_animation.bones.iter().find(|(j, _)| j == i) {
                    (
                        *i,
                        BoneTimeline::from_srts(
                            s.srt(current_time),
                            st.srt(start_offest),
                            duration,
                        ),
                    )
                } else {
                    (
                        *i,
                        BoneTimeline::from_srts(
                            s.srt(current_time),
                            s.srt(current_time + duration),
                            duration,
                        ),
                    )
                }
            })
            .collect();
        let slots = first_animation
            .slots
            .iter()
            .map(|(i, s)| {
                (
                    *i,
                    SlotTimeline::from_timelines(
                        s.interpolate_attachment(current_time)
                            .flatten()
                            .map(|x| x.to_owned()),
                    ),
                )
            })
            .collect();
        Animation {
            duration,
            slots,
            bones,
            events: first_animation.events.clone(),
            draworder: first_animation.draworder.clone(),
        }
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
