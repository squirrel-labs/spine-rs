use serde::{Deserialize, Deserializer};
use serde::de::{Error as SerdeError, Visitor};
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    //common
    pub name: Option<String>,
    pub type_: Option<AttachmentType>,
    //region
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
    pub rotation: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub fps: Option<f32>,
    pub mode: Option<String>,       // TODO: add enum forward, backward etc ...
    //mesh
    pub path: Option<String>,
    pub vertices: Option<Vec<f32>>,
    pub triangles: Option<Vec<u16>>,
    pub uvs: Option<Vec<f32>>,
    pub hull: Option<i32>,
    pub edges: Option<Vec<i32>>,
    #[serde(default = "white_color")]
    pub color: String
}

fn white_color() -> String { "FFFFFFFF".to_owned() }

#[derive(Debug, Clone)]
pub enum AttachmentType {
    Region,
    Mesh,
    RegionSequence,
    BoundingBox,
}

impl<'a> Deserialize<'a> for AttachmentType {
    fn deserialize<D>(deserializer: D) -> Result<AttachmentType, D::Error>
        where D: Deserializer<'a> {
        deserializer.deserialize_any(AttachmentTypeVisitor)
    }
}

struct AttachmentTypeVisitor;

impl<'a> Visitor<'a> for AttachmentTypeVisitor {
    type Value = AttachmentType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "one of (region, regionsequence, boundingbox, mesh)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: SerdeError {
        match value {
            "region" => Ok(AttachmentType::Region),
            "regionsequence" => Ok(AttachmentType::RegionSequence),
            "boundingbox" => Ok(AttachmentType::BoundingBox),
            "mesh" => Ok(AttachmentType::Mesh),
            _ => Err(SerdeError::custom(format!("Attachment type must be one of (region, regionsequence, boundingbox, mesh)")))
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: SerdeError {
        self.visit_str(value.as_ref())
    }
}
