use super::Bone;
use super::SkeletonError;
use super::Slot;

pub fn bone_index(name: &str, bones: &[Bone]) -> Result<usize, SkeletonError> {
    bones
        .iter()
        .position(|b| b.name == *name)
        .ok_or_else(|| SkeletonError::BoneNotFound(name.to_owned()))
}

pub fn slot_index(name: &str, slots: &[Slot]) -> Result<usize, SkeletonError> {
    slots
        .iter()
        .position(|b| b.name == *name)
        .ok_or_else(|| SkeletonError::SlotNotFound(name.to_owned()))
}
