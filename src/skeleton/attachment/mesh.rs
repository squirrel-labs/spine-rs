use json;

#[derive(Debug)]
pub struct MeshAttachment {
    pub name: Option<String>,
    pub triangles: Vec<u16>,
    edges: Vec<i32>,
    pub vertices: Vec<f32>,
    uvs: Vec<f32>,
    bone_indices: Option<Vec<u32>>,
    pub is_weighted: bool,
    pub world_vertices_length: u32,
}

impl MeshAttachment {
    pub fn new(attachment: json::Attachment, name: Option<String>) -> MeshAttachment {
        let mut mesh = MeshAttachment {
            name: attachment.name.or(name),
            triangles: attachment.triangles.unwrap_or(Vec::new()),
            edges: attachment.edges.unwrap_or(Vec::new()),
            vertices: attachment.vertices.unwrap_or(Vec::new()),
            uvs: attachment.uvs.unwrap_or(Vec::new()),
            bone_indices: None,
            is_weighted: false,
            world_vertices_length: 0,
        };

        mesh.update_vertices();
        mesh.update_uvs();
        mesh
    }

    fn update_vertices(&mut self) {
        let uvs_len = self.uvs.len();
        let is_weighted_mesh = self.vertices.len() > uvs_len;

        if !is_weighted_mesh {
            return;
        };

        self.is_weighted = is_weighted_mesh;
        let mut weights: Vec<f32> = Vec::with_capacity(uvs_len * 3 * 3);
        let mut bone_indices: Vec<u32> = Vec::with_capacity(uvs_len * 3);

        {
            let mut item_iter = self.vertices.iter();

            'items: loop {
                if let Some(bone_count) = item_iter.next() {
                    let mut bones_iter = item_iter.by_ref().take(*bone_count as usize * 4);
                    'bones: loop {
                        if let Some(bone_index) = bones_iter.next() {
                            let bind_x = bones_iter.next().copied().unwrap();
                            let bind_y = bones_iter.next().copied().unwrap();
                            let weight = bones_iter.next().copied().unwrap();

                            bone_indices.push(*bone_index as u32);
                            weights.push(bind_x);
                            weights.push(bind_y);
                            weights.push(weight);
                        } else {
                            break 'bones;
                        }
                    }
                } else {
                    break 'items;
                }
            }
        }

        self.vertices = weights;
        self.bone_indices = Some(bone_indices);
    }

    fn update_uvs(&self) {}
}
