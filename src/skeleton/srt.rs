use std::f32::consts::PI;

/// Scale, Rotate, Translate struct
#[derive(Debug, Clone)]
pub struct SRT {
    /// scale
    pub scale: [f32; 2],
    /// rotation in radians
    pub rotation: f32,
    /// position or translation
    pub position: [f32; 2],
    /// cosinus
    pub cos: f32,
    /// sinus
    pub sin: f32,
}

const TO_RADIAN: f32 = PI / 180f32;

impl SRT {
    /// new srt
    pub fn new(scale_x: f32, scale_y: f32, rotation_deg: f32, x: f32, y: f32) -> SRT {
        let rotation = rotation_deg * TO_RADIAN;
        SRT {
            scale: [scale_x, scale_y],
            rotation,
            position: [x, y],
            cos: rotation.cos(),
            sin: rotation.sin(),
        }
    }

    /// apply srt on a 2D point (consumes the point)
    pub fn transform(&self, v: [f32; 2]) -> [f32; 2] {
        [
            self.cos * v[0] * self.scale[0] - self.sin * v[1] * self.scale[1] + self.position[0],
            self.sin * v[0] * self.scale[0] + self.cos * v[1] * self.scale[1] + self.position[1],
        ]
    }

    /// convert srt to a 3x3 transformation matrix (2D)
    pub fn to_matrix3(&self) -> [[f32; 3]; 3] {
        [
            [self.cos * self.scale[0], self.sin * self.scale[0], 0.0],
            [-self.sin * self.scale[0], self.cos * self.scale[1], 0.0],
            [self.position[0], self.position[1], 1.0f32],
        ]
    }

    /// convert srt to a 4x4 transformation matrix (3D)
    pub fn to_matrix4(&self) -> [[f32; 4]; 4] {
        [
            [self.cos * self.scale[0], self.sin, 0.0, 0.0],
            [-self.sin, self.cos * self.scale[1], 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], 0.0, 1.0f32],
        ]
    }
}
