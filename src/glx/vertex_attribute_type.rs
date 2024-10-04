use ogl::types::{GLboolean, GLenum};
use crate::glx::vertex_attribute::VertexAttribute;

#[derive(Clone, Copy)]
pub enum VertexAttributeType {
    /// **3 components per position, float, not normalized**
    Position,
    /// **2 components per position, float, not normalized**
    Position2D,
    /// **4 components per color, float, not normalized**
    Color,
    /// **2 components per texture coordinate, float, not normalized**
    TexCoords,
    /// **3 components per normal, float, not normalized**
    Normal,
}

impl VertexAttributeType {
    pub fn components(&self) -> i32 {
        match self {
            VertexAttributeType::Position => 3,
            VertexAttributeType::Position2D => 2,
            VertexAttributeType::Color => 4,
            VertexAttributeType::TexCoords => 2,
            VertexAttributeType::Normal => 3
        }
    }

    pub fn data_type(&self) -> GLenum {
        gl::FLOAT
    }

    pub fn normalized(&self) -> GLboolean {
        gl::FALSE
    }

    pub fn to_gl_data(&self) -> (i32, GLenum, GLboolean) {
        (self.components(), self.data_type(), self.normalized())
    }

    /// Converts an input iterable of `VertexAttributeType` into a `Vec<VertexAttribute>`.
    /// # Arguments
    /// * `input` - An iterable of items that can be converted into VertexAttribute.
    pub fn convert_attributes<I>(input: I) -> Vec<VertexAttribute>
    where
        I: IntoIterator,
        I::Item: Into<VertexAttribute>,
    {
        input.into_iter().map(Into::into).collect()
    }
}

impl Into<GLenum> for VertexAttributeType {
    fn into(self) -> GLenum {
        self.data_type()
    }
}
