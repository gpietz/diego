use ogl::types::{GLint, GLsizei};
use crate::geometry::dimension::Dimension2D;
use crate::gl::color::Color;
use crate::gl::types::{Capability, GlGetParameter};
use crate::gl::GLConstant;
use crate::gl::state::get_integer_v_array;

/// Sets the OpenGL clear color for rendering.
///
/// This function specifies the red, green, blue, and alpha components of the
/// color that will be used when clearing the screen. The color is passed as a
/// `Color` struct, which contains `r`, `g`, `b`, and `a` values for the respective
/// color channels.
///
/// # Arguments
///
/// * `color` - A `Color` struct containing the red (`r`), green (`g`), blue (`b`),
///   and alpha (`a`) values, all of which are `f32`.
///
/// # Safety
///
/// This function calls the `gl::ClearColor` function from OpenGL, which is marked as
/// unsafe, because it interacts with the underlying graphics hardware.
/// Ensure that your OpenGL context is correctly set up before using this function.
///
/// # Example
/// ```no_run
/// let clear_color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }; // Set color to black
/// clear_color(clear_color);
/// ```
pub fn clear_color(color: Color) {
    unsafe {
        gl::ClearColor(color.r, color.g, color.b, color.a);
    }
}

/// Sets the OpenGL viewport with the specified position and dimensions.
///
/// This function defines the affine transformation of x and y from normalized device coordinates to
/// window coordinates. It sets the lower-left corner of the viewport rectangle and its width
/// and height. After setting the viewport, it checks for any OpenGL errors.
///
/// # Arguments
///
/// * `x` - The x-coordinate of the lower-left corner of the viewport.
/// * `y` - The y-coordinate of the lower-left corner of the viewport.
/// * `width` - The width of the viewport.
/// * `height` - The height of the viewport.
///
/// # Returns
///
/// * `true` - If the viewport was set successfully and no OpenGL errors occurred.
/// * `false` - If there was an OpenGL error during the viewport setup.
///
/// # Safety
///
/// This function is unsafe because it calls OpenGL functions, which require a valid OpenGL context.
/// Make sure the context is properly initialized before calling this function.
///
/// # Example
/// ```
/// if view_port(0.0, 0.0, 1920.0, 1080.0) {
///     println!("Viewport set successfully!");
/// } else {
///     println!("Failed to set the viewport.");
/// }
/// ```
///
/// This function casts the input values to the appropriate OpenGL types (`GLint` and `GLsizei`),
/// and checks for errors using `gl::GetError()`.
pub fn view_port(x: f32, y: f32, width: f32, height: f32) -> bool {
    unsafe {
        gl::Viewport(x as GLint, y as GLint, width as GLsizei, height as GLsizei);
        if gl::GetError() != gl::NO_ERROR {
            return false;
        }
        true
    }
}

/// Retrieves the maximum viewport dimensions supported by the OpenGL implementation.
///
/// This function calls `glGetIntegerv` with `GL_MAX_VIEWPORT_DIMS` to query the maximum
/// width and height that can be used for a viewport. The returned values are converted to `f32`
/// and returned as a `Dimension2D<f32>`.
///
/// # Returns
///
/// A `Dimension2D<f32>` containing the maximum width and height that can be used for a viewport.
///
/// # Safety
///
/// The function calls OpenGL's `gl::GetIntegerv`, which interacts with the underlying graphics
/// hardware and thus requires the OpenGL context to be correctly initialized before making
/// the call.
///
/// # Example
///
/// ```no_run
/// let max_dims = get_max_viewport_dims();
/// println!("Max viewport dimensions: {}x{}", max_dims.width, max_dims.height);
/// ```
///
/// The result will be a `Dimension2D<f32>` with the maximum viewport width and height.
pub fn get_max_viewport_dims() -> Dimension2D<f32> {
    let max_viewport_dims = get_integer_v_array(GlGetParameter::MaxViewPortDims, 2);
    Dimension2D::new(max_viewport_dims[0] as f32, max_viewport_dims[1] as f32)
}

pub fn enable(capability: Capability) {
    unsafe {
        gl::Enable(capability.to_gl_constant());
    }
}

pub fn disable(capability: Capability) {
    unsafe {
        gl::Disable(capability.to_gl_constant());
    }
}

pub fn is_enabled(capability: Capability) -> bool {
    unsafe { gl::IsEnabled(capability.to_gl_constant()) > 0 }
}
