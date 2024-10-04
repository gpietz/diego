use std::hash::{Hash, Hasher};
use crate::core::SizeOf;
use crate::glx::vertex_attribute_type::VertexAttributeType;
use crate::glx::vertex_data_type::VertexDataType;

#[derive(Clone, Debug)]
pub struct VertexAttribute {
    /// Optional name of the attribute, useful when querying by name in shader programs.
    pub name: Option<String>,
    pub components: u8,
    pub data_type: VertexDataType,
    pub normalized: bool,
    pub stride: i32,
    pub offset: Option<u32>,
}

impl VertexAttribute {
    pub fn new(components: u8, data_type: VertexDataType) -> Self {
        Self {
            name: None,
            components,
            data_type,
            normalized: false,
            stride: 0,
            offset: None,
        }
    }

    pub fn with_name<T: Into<Option<String>>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_components(mut self, components: u8) -> Self {
        self.components = components;
        self
    }

    pub fn with_data_type(mut self, data_type: VertexDataType) -> Self {
        self.data_type = data_type;
        self
    }

    pub fn with_normalized(mut self, normalized: bool) -> Self {
        self.normalized = normalized;
        self
    }

    pub fn with_stride(mut self, stride: i32) -> Self {
        self.stride = stride;
        self
    }

    pub fn with_offset(mut self, offset: Option<u32>) -> Self {
        self.offset = offset;
        self
    }

    /// Calculates the byte size of the attribute based on its specifications or its type.
    pub fn calculate_size(&self) -> usize {
        self.data_type.size() * self.components as usize
    }
}

impl From<VertexAttributeType> for VertexAttribute {
    fn from(value: VertexAttributeType) -> Self {
        match value {
            VertexAttributeType::Position => {
                VertexAttribute::new(3, VertexDataType::Float).with_name("position".to_string())
            }
            VertexAttributeType::Position2D => {
                VertexAttribute::new(2, VertexDataType::Float).with_name("position".to_string())
            }
            VertexAttributeType::Color => {
                VertexAttribute::new(4, VertexDataType::Float).with_name("color".to_string())
            }
            VertexAttributeType::TexCoords => {
                VertexAttribute::new(2, VertexDataType::Float).with_name("tex_coord".to_string())
            }
            VertexAttributeType::Normal => {
                VertexAttribute::new(3, VertexDataType::Float).with_name("normal".to_string())
            }
        }
    }
}

impl Hash for VertexAttribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.components.hash(state);
        self.data_type.hash(state);
        self.normalized.hash(state);
    }
}

impl PartialEq<Self> for VertexAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
            && self.data_type == other.data_type
            && self.normalized == other.normalized
    }
}

impl Eq for VertexAttribute {}
