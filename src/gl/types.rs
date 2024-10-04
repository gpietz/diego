/// Represents OpenGL capabilities that can be enabled or disabled.
pub enum Capability {
    /// Capability to blend pixels.
    Blend,
    /// Capability to perform depth comparisons and update the depth buffer.
    DepthTest,
    /// Capability to cull polygons based on their winding in window coordinates.
    CullFace,
    /// Capability to perform scissor test, that is to discard fragments that are
    /// outside of the scissor rectangle.
    ScissorTest,
    /// Capability to use dithering when merging fragment colors and depth values.
    Dither,
    /// Capability for line smoothing.
    LineSmooth,
    /// Capability for polygon smoothing.
    PolygonSmooth,
    /// Capability to update stencil buffer.
    StencilTest,
}

impl GLConstant for Capability {
    /// Converts the capability to its corresponding OpenGL enum value.
    fn to_gl_constant(self) -> GLuint {
        match self {
            Capability::Blend => gl::BLEND,
            Capability::DepthTest => gl::DEPTH_TEST,
            Capability::CullFace => gl::CULL_FACE,
            Capability::ScissorTest => gl::SCISSOR_TEST,
            Capability::Dither => gl::DITHER,
            Capability::LineSmooth => gl::LINE_SMOOTH,
            Capability::PolygonSmooth => gl::POLYGON_SMOOTH,
            Capability::StencilTest => gl::STENCIL_TEST,
        }
    }
}

mod clear_buffer_mask {
    bitflags::bitflags! {
       pub struct ClearBufferMask: u32 {
            const COLOR_BUFFER = gl::COLOR_BUFFER_BIT;
            const DEPTH_BUFFER = gl::DEPTH_BUFFER_BIT;
            const STENCIL_BUFFER = gl::STENCIL_BUFFER_BIT;
        }
    }
}

use ogl::types::{GLenum, GLuint};
pub use clear_buffer_mask::ClearBufferMask;
use crate::gl::GLConstant;

#[repr(u32)]
pub enum GlGetParameter {
    MaxTextureSize = gl::MAX_TEXTURE_SIZE,
    MaxVertexAttribs = gl::MAX_VERTEX_ATTRIBS,
    Viewport = gl::VIEWPORT,
    MaxCombinedTextureImageUnits = gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS,
    MaxTextureImageUnits = gl::MAX_TEXTURE_IMAGE_UNITS,
    MaxVertexTextureImageUnits = gl::MAX_VERTEX_TEXTURE_IMAGE_UNITS,
    MaxFragmentUniformComponents = gl::MAX_FRAGMENT_UNIFORM_COMPONENTS,
    MaxVertexUniformComponents = gl::MAX_VERTEX_UNIFORM_COMPONENTS,
    MaxVaryingComponents = gl::MAX_VARYING_COMPONENTS,
    MaxDrawBuffers = gl::MAX_DRAW_BUFFERS,
    MaxElementsIndices = gl::MAX_ELEMENTS_INDICES,
    MaxElementsVertices = gl::MAX_ELEMENTS_VERTICES,
    Max3DTextureSize = gl::MAX_3D_TEXTURE_SIZE,
    VertexArrayBinding = gl::VERTEX_ATTRIB_BINDING,
    ArrayBufferBinding = gl::ARRAY_BUFFER_BINDING,
}

impl From<GLenum> for GlGetParameter {
    fn from(value: GLenum) -> Self {
        match value {
            gl::MAX_TEXTURE_SIZE => GlGetParameter::MaxTextureSize,
            gl::MAX_VERTEX_ATTRIBS => GlGetParameter::MaxVertexAttribs,
            gl::VIEWPORT => GlGetParameter::Viewport,
            gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS => GlGetParameter::MaxCombinedTextureImageUnits,
            gl::MAX_TEXTURE_IMAGE_UNITS => GlGetParameter::MaxTextureImageUnits,
            gl::MAX_VERTEX_TEXTURE_IMAGE_UNITS => GlGetParameter::MaxVertexTextureImageUnits,
            gl::MAX_FRAGMENT_UNIFORM_COMPONENTS => GlGetParameter::MaxFragmentUniformComponents,
            gl::MAX_VERTEX_UNIFORM_COMPONENTS => GlGetParameter::MaxVertexUniformComponents,
            gl::MAX_VARYING_COMPONENTS => GlGetParameter::MaxVaryingComponents,
            gl::MAX_DRAW_BUFFERS => GlGetParameter::MaxDrawBuffers,
            gl::MAX_ELEMENTS_INDICES => GlGetParameter::MaxElementsIndices,
            gl::MAX_ELEMENTS_VERTICES => GlGetParameter::MaxElementsVertices,
            gl::MAX_3D_TEXTURE_SIZE => GlGetParameter::Max3DTextureSize,
            gl::VERTEX_ARRAY_BINDING => GlGetParameter::VertexArrayBinding,
            gl::ARRAY_BUFFER_BINDING => GlGetParameter::ArrayBufferBinding,
            _ => panic!("Unknown GLenum: {}", value),
        }
    }
}

impl From<GlGetParameter> for GLenum {
    fn from(value: GlGetParameter) -> Self {
        match value {
            GlGetParameter::MaxTextureSize => gl::MAX_TEXTURE_SIZE,
            GlGetParameter::MaxVertexAttribs => gl::MAX_VERTEX_ATTRIBS,
            GlGetParameter::Viewport => gl::VIEWPORT,
            GlGetParameter::MaxCombinedTextureImageUnits => gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS,
            GlGetParameter::MaxTextureImageUnits => gl::MAX_TEXTURE_IMAGE_UNITS,
            GlGetParameter::MaxVertexTextureImageUnits => gl::MAX_VERTEX_TEXTURE_IMAGE_UNITS,
            GlGetParameter::MaxFragmentUniformComponents => gl::MAX_FRAGMENT_UNIFORM_COMPONENTS,
            GlGetParameter::MaxVertexUniformComponents => gl::MAX_VERTEX_UNIFORM_COMPONENTS,
            GlGetParameter::MaxVaryingComponents => gl::MAX_VARYING_COMPONENTS,
            GlGetParameter::MaxDrawBuffers => gl::MAX_DRAW_BUFFERS,
            GlGetParameter::MaxElementsIndices => gl::MAX_ELEMENTS_INDICES,
            GlGetParameter::MaxElementsVertices => gl::MAX_ELEMENTS_VERTICES,
            GlGetParameter::Max3DTextureSize => gl::MAX_3D_TEXTURE_SIZE,
            GlGetParameter::VertexArrayBinding => gl::VERTEX_ARRAY_BINDING,
            GlGetParameter::ArrayBufferBinding => gl::ARRAY_BUFFER_BINDING,
        }
    }
}

pub enum BufferType {
    /// Stores vertex attributes like vertex coordinates, normals, texture coordinates, etc.
    ArrayBuffer,
    /// Used for indexing vertices, allowing the reuse of vertex data for multiple primitives.
    ElementArrayBuffer,
    /// Stores uniform data for shaders. This allows for efficient sharing of data between
    /// multiple shaders.
    UniformBuffer,
    /// Stores data for a buffer texture, a special texture type that is accessed with a texel
    /// fetch operation in GLSL.
    TextureBuffer,
    /// Not a buffer in the traditional sense, but a target that contains attachments like color,
    /// depth, and stencil buffers for rendering.
    Framebuffer,
    /// Used as a rendering destination, typically for offscreen rendering, and can store data
    /// like color, depth, and stencil.
    Renderbuffer,
    /// Used for copying data between buffers.
    CopyReadBuffer,
    /// Used for copying data between buffers.
    CopyWriteBuffer,
    /// Used in pixel transfer operations, like reading from or writing to textures.
    PixelPackBuffer,
    /// Used in pixel transfer operations, like reading from or writing to textures.
    PixelUnpackBuffer,
    /// Captures output from the vertex shader or geometry shader.
    TransformFeedbackBuffer,
    /// Stores atomic counters, used for achieving synchronization and consistency across
    /// shader invocations.
    AtomicCounterBuffer,
    /// Used for storing indirect drawing commands.
    DrawIndirectBuffer,
    /// Similar to the draw indirect buffer, but used for compute shader dispatch commands.
    DispatchIndirectBuffer,
    /// Provides read-write storage for data that is accessed by shaders. It's more flexible
    /// compared to uniform buffers.
    ShaderStorageBuffer,
}

impl GLConstant for BufferType {
    fn to_gl_constant(self) -> GLuint {
        match self {
            BufferType::ArrayBuffer => gl::ARRAY_BUFFER,
            BufferType::ElementArrayBuffer => gl::ELEMENT_ARRAY_BUFFER,
            BufferType::UniformBuffer => gl::UNIFORM_BUFFER,
            BufferType::TextureBuffer => gl::TEXTURE_BUFFER,
            BufferType::Framebuffer => gl::FRAMEBUFFER,
            BufferType::Renderbuffer => gl::RENDERBUFFER,
            BufferType::CopyReadBuffer => gl::COPY_READ_BUFFER,
            BufferType::CopyWriteBuffer => gl::COPY_WRITE_BUFFER,
            BufferType::PixelPackBuffer => gl::PIXEL_PACK_BUFFER,
            BufferType::PixelUnpackBuffer => gl::PIXEL_UNPACK_BUFFER,
            BufferType::TransformFeedbackBuffer => gl::TRANSFORM_FEEDBACK_BUFFER,
            BufferType::AtomicCounterBuffer => gl::ATOMIC_COUNTER_BUFFER,
            BufferType::DrawIndirectBuffer => gl::DRAW_INDIRECT_BUFFER,
            BufferType::DispatchIndirectBuffer => gl::DISPATCH_INDIRECT_BUFFER,
            BufferType::ShaderStorageBuffer => gl::SHADER_STORAGE_BUFFER,
        }
    }
}

impl From<GLenum> for BufferType {
    fn from(value: GLenum) -> Self {
        match value {
            gl::ARRAY_BUFFER => BufferType::ArrayBuffer,
            gl::ELEMENT_ARRAY_BUFFER => BufferType::ElementArrayBuffer,
            gl::UNIFORM_BUFFER => BufferType::UniformBuffer,
            gl::TEXTURE_BUFFER => BufferType::TextureBuffer,
            gl::FRAMEBUFFER => BufferType::Framebuffer,
            gl::RENDERBUFFER => BufferType::Renderbuffer,
            gl::COPY_READ_BUFFER => BufferType::CopyReadBuffer,
            gl::COPY_WRITE_BUFFER => BufferType::CopyWriteBuffer,
            gl::PIXEL_PACK_BUFFER => BufferType::PixelPackBuffer,
            gl::PIXEL_UNPACK_BUFFER => BufferType::PixelUnpackBuffer,
            gl::TRANSFORM_FEEDBACK_BUFFER => BufferType::TransformFeedbackBuffer,
            gl::ATOMIC_COUNTER_BUFFER => BufferType::AtomicCounterBuffer,
            gl::DRAW_INDIRECT_BUFFER => BufferType::DrawIndirectBuffer,
            gl::DISPATCH_INDIRECT_BUFFER => BufferType::DispatchIndirectBuffer,
            gl::SHADER_STORAGE_BUFFER => BufferType::ShaderStorageBuffer,
            _ => panic!("Unknown GLenum: {}", value),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BufferUsage {
    /// Used when the data in the buffer will not change or will change only infrequently and is
    /// used primarily for drawing.
    StaticDraw,
    /// Used when the data in the buffer will change frequently and is used primarily for drawing.
    DynamicDraw,
    /// Used when the data in the buffer will change on every draw and is used primarily
    /// for drawing.
    StreamDraw,
    /// Used for buffers that are not changed by the application and are read by the GPU.
    StaticRead,
    /// Used for buffers that are changed frequently by the application and are read by the GPU.
    DynamicRead,
    /// Used for buffers that are changed and read by the GPU once.
    StreamRead,
    /// Used for buffers that are not changed by the application but are used for copying data
    /// from one buffer to another.
    StaticCopy,
    /// Used for buffers that are changed frequently and used for copying data from one buffer
    /// to another.
    DynamicCopy,
    /// sed for buffers that are changed and used for copying data from one buffer to another once.
    StreamCopy,
}

impl GLConstant for BufferUsage {
    fn to_gl_constant(self) -> GLuint {
        match self {
            BufferUsage::StaticDraw => gl::STATIC_DRAW,
            BufferUsage::DynamicDraw => gl::DYNAMIC_DRAW,
            BufferUsage::StreamDraw => gl::STREAM_DRAW,
            BufferUsage::StaticRead => gl::STATIC_READ,
            BufferUsage::DynamicRead => gl::DYNAMIC_READ,
            BufferUsage::StreamRead => gl::STREAM_READ,
            BufferUsage::StaticCopy => gl::STATIC_COPY,
            BufferUsage::DynamicCopy => gl::DYNAMIC_COPY,
            BufferUsage::StreamCopy => gl::STREAM_COPY,
        }
    }
}

