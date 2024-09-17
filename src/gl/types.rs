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

impl Capability {
    /// Converts the capability to its corresponding OpenGL enum value.
    pub fn to_gl_constant(self) -> u32 {
        match self {
            Capability::Blend => gl::BLEND,
            Capability::DepthTest => gl::DEPTH_TEST,
            Capability::CullFace => gl::CULL_FACE,
            Capability::ScissorTest =>  gl::SCISSOR_TEST,
            Capability::Dither =>  gl::DITHER,
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

pub use clear_buffer_mask::ClearBufferMask;
