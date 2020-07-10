pub mod mesh;
pub mod region;

use self::mesh::MeshAttachment;
use self::region::RegionAttachment;
use super::json;
use super::json::AttachmentType;

#[derive(Debug)]
pub enum Attachment {
    Region(RegionAttachment),
    Mesh(MeshAttachment),
}

#[derive(Debug)]
pub enum AttachmentError {
    UnknownType,
}

impl Attachment {
    pub fn name(&self) -> Option<&String> {
        match self {
            Attachment::Region(region) => region.name.as_ref(),
            Attachment::Mesh(mesh) => mesh.name.as_ref(),
        }
    }
    /// converts json data into skeleton data
    pub fn from_json(
        attachment: json::Attachment,
        name: Option<String>,
    ) -> Result<Attachment, AttachmentError> {
        let t = attachment.type_.clone();

        match t.unwrap_or(AttachmentType::Region) {
            AttachmentType::Region => {
                Ok(Attachment::Region(RegionAttachment::new(attachment, name)))
            }
            AttachmentType::Mesh => Ok(Attachment::Mesh(MeshAttachment::new(attachment, name))),
            _ => Err(AttachmentError::UnknownType),
        }
    }
}

