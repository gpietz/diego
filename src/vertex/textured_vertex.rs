use crate::glx::vertex_attribute::VertexAttribute;
use crate::glx::vertex_attribute_type::VertexAttributeType;
use crate::glx::vertex_layout::VertexLayout;
use crate::vertex::VertexEq;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TexturedVertex {
    pub position: [f32; 3],   // XYZ coordinates
    pub tex_coords: [f32; 2], // UV texture coordinates
    pub color: [f32; 4],      // color of the vertex
}

impl Default for TexturedVertex {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl VertexLayout for TexturedVertex {
    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttributeType::Position.into(),
            VertexAttributeType::TexCoords.into(),
            VertexAttributeType::Color.into(),
        ]
    }
}

impl VertexEq<TexturedVertex> for TexturedVertex {
    fn is_similar(&self, other: &TexturedVertex, tolerance: f32) -> bool {
        self.position
            .iter()
            .zip(other.position.iter())
            .all(|(a, b)| (a - b).abs() <= tolerance)
            && self
            .tex_coords
            .iter()
            .zip(other.tex_coords.iter())
            .all(|(a, b)| (a - b).abs() <= tolerance)
            && self
            .color
            .iter()
            .zip(other.color.iter())
            .all(|(a, b)| (a - b).abs() <= tolerance)
    }
}

impl TexturedVertex {
    pub fn new_xyz_uv(x: f32, y: f32, z: f32, u: f32, v: f32) -> Self {
        Self {
            position: [x, y, z],
            tex_coords: [u, v],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}
