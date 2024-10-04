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
        gl::GetIntegerv(get_parameter.into(), &mut value);
        value
    }
}
