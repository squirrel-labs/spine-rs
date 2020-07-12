use super::json;
use super::srt::SRT;
use super::util;
use super::SkeletonError;

/// skeleton bone
pub struct Bone {
    pub name: String,
    pub parent_index: Option<usize>,
    // length: f32,
    pub srt: SRT,
    pub inherit_scale: bool,
    pub inherit_rotation: bool,
    pub transform_mode: TransformMode,
}

impl Bone {
    pub fn from_json(bone: json::Bone, bones: &[Bone]) -> Result<Bone, SkeletonError> {
        let index = match bone.parent {
            Some(ref name) => Some(util::bone_index(name, bones)?),
            None => None,
        };
        Ok(Bone {
            transform_mode: bone
                .transform
                .map_or(TransformMode::Normal, |mode| TransformMode::from(mode)),
            name: bone.name,
            parent_index: index,
            // length: bone.length.unwrap_or(0f32),
            srt: SRT::new(
                bone.scale_x.unwrap_or(1.0),
                bone.scale_y.unwrap_or(1.0),
                bone.rotation.unwrap_or(0.0),
                bone.x.unwrap_or(0.0),
                bone.y.unwrap_or(0.0),
            ),
            inherit_scale: bone.inherit_scale.unwrap_or(true),
            inherit_rotation: bone.inherit_rotation.unwrap_or(true),
        })
    }
}

pub enum TransformMode {
    Normal,
    OnlyTranslation,
    NoRotationOrReflection,
    NoScaleOrReflection,
    NoScale,
}

impl From<String> for TransformMode {
    fn from(mode: String) -> TransformMode {
        println!("{}", mode);
        match &*mode {
            "onlyTranslation" => TransformMode::OnlyTranslation,
            "noRotationOrReflection" => TransformMode::NoRotationOrReflection,
            "noScaleOrReflection" => TransformMode::NoScaleOrReflection,
            "noScale" => TransformMode::NoScale,
            _ => TransformMode::Normal,
        }
    }
}
