use crate::gl::types::GlGetParameter;

/// `get_integer_v` is a utility function that retrieves the value of a specific OpenGL
/// parameter as an integer.
/// # Parameters
/// * `get_parameter: GlGetParameter` - An enum or type representing the OpenGL parameter to
///    retrieve. This value is converted into a format that the OpenGL function `glGetIntegerv`
///    expects.
pub fn get_integer_v(get_parameter: GlGetParameter) -> i32 {
    unsafe {
        let mut value = 0;
        gl::GetIntegerv(get_parameter.into(), &mut value as *mut _);
        value
    }
}

/// Retrieves an array of integer values from OpenGL using the specified parameter.
///
/// This function uses `gl::GetIntegerv` to query multiple integer values from OpenGL,
/// such as the maximum viewport dimensions or other configuration limits. The number
/// of values returned is determined by the `size` argument.
///
/// # Arguments
///
/// * `gl_get_parameter` - The OpenGL parameter to query, represented by the `GlGetParameter` enum.
/// * `size` - The number of integer values to retrieve, which determines
///   the size of the returned array.
///
/// # Returns
///
/// A `Vec<i32>` containing the integer values retrieved from OpenGL. The size of the vector is
/// determined by the `size` argument.
///
/// # Safety
///
/// The function calls `gl::GetIntegerv`, which interacts with the underlying graphics hardware.
/// Ensure that an OpenGL context is correctly initialized before calling this function.
///
/// # Example
/// ```no_run
/// let viewport_dims = get_integer_v_array(GlGetParameter::MaxViewportDims, 2);
/// println!("Max viewport dimensions: {}x{}", viewport_dims[0], viewport_dims[1]);
/// ```
///
/// In this example, the maximum width and height supported by the OpenGL viewport are queried.
///
/// # Notes
///
/// This function assumes that the `size` argument correctly matches the number of values that the
/// queried OpenGL parameter will return.
pub fn get_integer_v_array(gl_get_parameter: GlGetParameter, size: usize) -> Vec<i32> {
    let mut values = vec![0; size];
    unsafe {
        gl::GetIntegerv(gl_get_parameter.into(), values.as_mut_ptr() as *mut _);
    }
    values
}
