use serde::{Deserialize, Deserializer};
use serde::de::{Error as SerdeError, Visitor};
use std::fmt;

#[derive(Debug, Clone)]
pub enum AttachmentType {
    Region,
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
        write!(formatter, "one of (region, regionsequence, boundingbox)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: SerdeError {
        match value {
            "region" => Ok(AttachmentType::Region),
            "regionsequence" => Ok(AttachmentType::RegionSequence),
            "boundingbox" => Ok(AttachmentType::BoundingBox),
            _ => Err(SerdeError::custom(format!("Attachment type must be one of (region, regionsequence, boundingbox)")))
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: SerdeError {
        self.visit_str(value.as_ref())
    }
}
