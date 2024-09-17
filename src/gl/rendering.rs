use crate::gl::types::ClearBufferMask;

/// Clears the color and depth buffers of the OpenGL context.
///
/// This function uses the `gl::Clear` call to clear both the color buffer
/// and the depth buffer. It ensures that the screen is ready for new
/// drawing operations by clearing any previous content.
///
/// # Safety
/// The function involves an unsafe block due to the interaction with
/// the OpenGL C API. Make sure that the OpenGL context is correctly
/// initialized and active when this function is called.
///
/// # Example
/// ```
/// use diego::gl::rendering::clear;
/// 
/// clear();
/// ```
/// This will clear both the color and depth buffers, preparing the screen for the next render pass.
pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

/// Clears the specified buffers of the OpenGL context based on the provided mask.
///
/// This function allows more flexibility by accepting a `ClearBufferMask` that 
/// defines which buffers to clear. The buffers could include the color buffer, 
/// depth buffer, stencil buffer, etc., depending on the flags set in the mask.
///
/// # Parameters
/// - `mask`: A bitmask of type `ClearBufferMask` that specifies which buffers 
///   should be cleared. This could be a combination of buffers like 
///   `COLOR_BUFFER`, `DEPTH_BUFFER`, and `STENCIL_BUFFER`.
///
/// # Safety
/// The function involves an unsafe block due to the interaction with the 
/// OpenGL C API. Ensure the OpenGL context is properly initialized and active 
/// when this function is called.
///
/// # Example
/// ```
/// use diego::gl::rendering::clear_with_mask;
/// use diego::gl::types::ClearBufferMask;
///
/// let mask = ClearBufferMask::COLOR_BUFFER | ClearBufferMask::DEPTH_BUFFER;
/// clear_with_mask(mask);
/// ```
/// This will clear both the color and depth buffers based on the provided mask.
pub fn clear_with_mask(mask: ClearBufferMask) {
    unsafe {
        gl::Clear(mask.bits());
    }
}
