use std::fmt;
use std::fmt::Display;
use ogl::types::GLenum;
use VertexDataType::Float;
use crate::core::{ConversionError, SizeOf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexDataType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    HalfFloat,
    Float,
    Double,
    Fixed,
    #[allow(non_camel_case_types)]
    Int_2_10_10_10_Rev,
    #[allow(non_camel_case_types)]
    UnsignedInt_2_10_10_10_Rev,
    #[allow(non_camel_case_types)]
    UnsignedInt_10F_11F_11F_Rev,
}

impl TryFrom<GLenum> for VertexDataType {
    type Error = ConversionError<GLenum>;

    fn try_from(value: GLenum) -> Result<Self, Self::Error> {
        match value {
            gl::BYTE => Ok(VertexDataType::Byte),
            gl::UNSIGNED_BYTE => Ok(VertexDataType::UnsignedByte),
            gl::SHORT => Ok(VertexDataType::Short),
            gl::UNSIGNED_SHORT => Ok(VertexDataType::UnsignedShort),
            gl::INT => Ok(VertexDataType::Int),
            gl::UNSIGNED_INT => Ok(VertexDataType::UnsignedInt),
            gl::HALF_FLOAT => Ok(VertexDataType::HalfFloat),
            gl::FLOAT => Ok(Float),
            gl::DOUBLE => Ok(VertexDataType::Double),
            gl::FIXED => Ok(VertexDataType::Fixed),
            gl::INT_2_10_10_10_REV => Ok(VertexDataType::Int_2_10_10_10_Rev),
            gl::UNSIGNED_INT_2_10_10_10_REV => Ok(VertexDataType::UnsignedInt_2_10_10_10_Rev),
            gl::UNSIGNED_INT_10F_11F_11F_REV => Ok(VertexDataType::UnsignedInt_10F_11F_11F_Rev),
            _ => Err(ConversionError::InvalidValue(value)),
        }
    }
}

impl Into<GLenum> for VertexDataType {
    fn into(self) -> GLenum {
        match self {
            VertexDataType::Byte => gl::BYTE,
            VertexDataType::UnsignedByte => gl::UNSIGNED_BYTE,
            VertexDataType::Short => gl::SHORT,
            VertexDataType::UnsignedShort => gl::UNSIGNED_SHORT,
            VertexDataType::Int => gl::INT,
            VertexDataType::UnsignedInt => gl::UNSIGNED_INT,
            VertexDataType::HalfFloat => gl::HALF_FLOAT,
            Float => gl::FLOAT,
            VertexDataType::Double => gl::DOUBLE,
            VertexDataType::Fixed => gl::FIXED,
            VertexDataType::Int_2_10_10_10_Rev => gl::INT_2_10_10_10_REV,
            VertexDataType::UnsignedInt_2_10_10_10_Rev => gl::UNSIGNED_INT_2_10_10_10_REV,
            VertexDataType::UnsignedInt_10F_11F_11F_Rev => gl::UNSIGNED_INT_10F_11F_11F_REV,
        }
    }
}

impl SizeOf for VertexDataType {
    fn size(&self) -> usize {
        match self {
            VertexDataType::Byte => size_of::<i8>(),
            VertexDataType::UnsignedByte => size_of::<u8>(),
            VertexDataType::Short => size_of::<i16>(),
            VertexDataType::UnsignedShort => size_of::<u16>(),
            VertexDataType::Int => size_of::<i32>(),
            VertexDataType::UnsignedInt => size_of::<u32>(),
            VertexDataType::HalfFloat => 2,  // Spezifische Größe
            Float => size_of::<f32>(),
            VertexDataType::Double => size_of::<f64>(),
            VertexDataType::Fixed => 4,  // 32-Bit Fixed
            VertexDataType::Int_2_10_10_10_Rev => 4,  // 32-Bit packed
            VertexDataType::UnsignedInt_2_10_10_10_Rev => 4,  // 32-Bit packed
            VertexDataType::UnsignedInt_10F_11F_11F_Rev => 4,  // 32-Bit packed
        }
    }
}

impl Display for VertexDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VertexDataType::Byte => write!(f, "Byte"),
            VertexDataType::UnsignedByte => write!(f, "UnsignedByte"),
            VertexDataType::Short => write!(f, "Short"),
            VertexDataType::UnsignedShort => write!(f, "UnsignedShort"),
            VertexDataType::Int => write!(f, "Int"),
            VertexDataType::UnsignedInt => write!(f, "UnsignedInt"),
            VertexDataType::HalfFloat => write!(f, "HalfFloat"),
            Float => write!(f, "Float"),
            VertexDataType::Double => write!(f, "Double"),
            VertexDataType::Fixed => write!(f, "Fixed"),
            VertexDataType::Int_2_10_10_10_Rev => write!(f, "Int_2_10_10_10_Rev"),
            VertexDataType::UnsignedInt_2_10_10_10_Rev => write!(f, "UnsignedInt_2_10_10_10_Rev"),
            VertexDataType::UnsignedInt_10F_11F_11F_Rev => write!(f, "UnsignedInt_10F_11F_11F_Rev"),
        }
    }
}
