use crate::glx::vertex_attribute_type::VertexAttributeType;
use ogl::types::GLboolean;
use std::ffi::c_void;
use std::ptr;

/// Generates and returns a new OpenGL Vertex Array Object (VAO).
/// # Returns
/// * `u32` - The ID of the newly generated VAO.
pub fn gen_vertex_array() -> u32 {
    unsafe {
        let mut vao_id = 0;
        gl::GenVertexArrays(1, &mut vao_id);
        vao_id
    }
}

/// Deletes an OpenGL Vertex Array Object (VAO) given its ID.
/// # Parameters
/// * `vao` - The ID of the VAO to be deleted.
pub fn delete_vertex_array(vao: u32) {
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
    }
}

/// Binds the specified OpenGL Vertex Array Object (VAO) for subsequent rendering operations.
/// # Parameters
/// * `vao` - The ID of the VAO to bind. Passing `0` will unbind the currently bound VAO.
pub fn bind_vertex_array(vao: u32) {
    unsafe {
        gl::BindVertexArray(vao);
    }
}

/// Specifies the format of the vertex attribute data for the currently bound vertex buffer.
///
/// This function sets the pointer to the attribute array, specifying how data is stored in the
/// vertex buffer and how it should be interpreted by OpenGL during rendering.
///
/// This function sets the format for vertex attributes in OpenGL, allowing you to define how
/// vertex data is interpreted during rendering.
///
/// # Parameters
///
/// * `index` - The index of the generic vertex attribute to modify.
/// * `size` - The number of components per attribute
///    (e.g., 1 for a float, 2 for a vec2, 3 for a vec3).
/// * `attribute_type` - The data type of each component in the attribute
///    (e.g., `GL_FLOAT`, `GL_INT`).
/// * `normalized` - Whether fixed-point data values should be normalized (`true`) or not (`false`)
///    when accessed.
/// * `stride` - The byte offset between consecutive attributes. If set to 0, the attributes are
///    tightly packed.
/// * `ptr` - A pointer to the first component of the first attribute in the array. If `None`,
///    it defaults to a null pointer.
pub fn vertex_attrib_pointer(index: u32,
                             size: i32,
                             attribute_type: VertexAttributeType,
                             normalized: bool,
                             stride: i32,
                             ptr: Option<*const c_void>) {
    unsafe {
        gl::VertexAttribPointer(
            index,
            size,
            attribute_type.into(),
            normalized as GLboolean,
            stride,
            ptr.unwrap_or_else(|| ptr::null())
        );
    }
}

/// Specifies the format of the integer vertex attribute data for the currently bound vertex buffer.
///
/// This function sets the pointer to the integer attribute array, specifying how data is stored in
/// the vertex buffer and how it should be interpreted by OpenGL during rendering.
/// Unlike `vertex_attrib_pointer`, this function is used specifically for non-floating-point
/// (integer) attributes.
///
/// This function is used to define integer vertex attributes for rendering in OpenGL, where
/// each attribute contains non-floating-point data (e.g., integers). Make sure to use it when
/// handling integer-based attributes.
///
/// # Parameters
///
/// * `index` - The index of the generic vertex attribute to modify.
/// * `size` - The number of components per attribute
///    (e.g., 1 for a single integer, 2 for a pair, 3 for a triplet).
/// * `attribute_type` - The data type of each component in the attribute
///    (e.g., `GL_INT`, `GL_UNSIGNED_INT`).
/// * `stride` - The byte offset between consecutive attributes. If set to 0, the attributes are
///    tightly packed.
/// * `ptr` - A pointer to the first component of the first attribute in the array. If `None`,
///    it defaults to a null pointer.
///
pub fn vertex_attrib_pointer_i(index: u32,
                               size: i32,
                               attribute_type: VertexAttributeType,
                               stride: i32,
                               ptr: Option<*const c_void>) {
    unsafe {
        gl::VertexAttribIPointer(
            index,
            size,
            attribute_type.into(),
            stride,
            ptr.unwrap_or_else(|| ptr::null())
        );
    }
}

/// Specifies the format of the double-precision vertex attribute data for the currently bound
/// vertex buffer.
///
/// This function sets the pointer to the array of double-precision vertex attributes, specifying
/// how data is stored in the vertex buffer and how it should be interpreted by OpenGL during
/// rendering. This is used specifically for `double`-precision floating point attributes.
///
/// This function defines how double-precision vertex attributes should be interpreted during
/// OpenGL rendering. Make sure to use it for attributes with double-precision floating point data
/// (GL_DOUBLE).
///
/// # Parameters
///
/// * `index` - The index of the generic vertex attribute to modify.
/// * `size` - The number of components per attribute
///    (e.g., 1 for a single double, 2 for a vec2, 3 for a vec3).
/// * `attribute_type` - The data type of each component in the attribute
///    (should typically be `GL_DOUBLE`).
/// * `stride` - The byte offset between consecutive attributes. If set to 0, the attributes are
///    tightly packed.
/// * `ptr` - A pointer to the first component of the first attribute in the array.
///    If `None`, it defaults to a null pointer.
pub fn vertex_attrib_pointer_l(index: u32,
                               size: i32,
                               attribut_type: VertexAttributeType,
                               stride: i32,
                               ptr: Option<*const c_void>) {
    unsafe {
        gl::VertexAttribLPointer(
            index,
            size,
            attribut_type.into(),
            stride,
            ptr.unwrap_or_else(|| ptr::null())
        );
    }
}

/// Enables a generic vertex attribute array at the specified index.
/// This function activates a vertex attribute array, allowing OpenGL to use the attribute data
/// during rendering. The index corresponds to the generic vertex attribute that
/// needs to be enabled.
/// # Parameters
/// * `index` - The index of the vertex attribute to enable.
pub fn enable_vertex_attrib_array(index: u32) {
    unsafe {
        gl::EnableVertexAttribArray(index);
    }
}

/// Disables a generic vertex attribute array at the specified index.
/// This function deactivates a vertex attribute array, preventing OpenGL from using the attribute
/// data during rendering. The index corresponds to the generic vertex attribute that needs
/// to be disabled.
/// # Parameters
/// * `index` - The index of the vertex attribute to disable.
pub fn disable_vertex_attrib_array(index: u32) {
    unsafe {
        gl::DisableVertexAttribArray(index);
    }
}

/// Enables a specific vertex attribute in a given Vertex Array Object (VAO).
///
/// This function enables the vertex attribute at the specified `index` within the provided `vao`.
/// Once enabled, OpenGL will use the specified attribute for rendering operations when
/// the VAO is bound.
///
/// This function enables a specific vertex attribute in a given VAO, making it active for
/// subsequent OpenGL rendering operations. Ensure that both the VAO and the attribute index are
/// valid.
///
/// # Parameters
///
/// * `vao` - The ID of the Vertex Array Object (VAO) that contains the attribute to enable.
/// * `index` - The index of the vertex attribute within the VAO to enable.
pub fn enable_vertex_array_attrib(vao: u32, index: u32) {
    unsafe {
        gl::EnableVertexArrayAttrib(vao, index);
    }
}

/// Disables a specific vertex attribute in a given Vertex Array Object (VAO).
/// 
/// This function disables the vertex attribute at the specified `index` within the provided `vao`.
/// Once disabled, OpenGL will no longer use the specified attribute for rendering operations when 
/// the VAO is bound.
/// 
/// # Parameters
/// * `vao` - The ID of the Vertex Array Object (VAO) that contains the attribute to disable.
/// * `index` - The index of the vertex attribute within the VAO to disable.
pub fn disable_vertex_array_attrib(vao: u32, index: u32) {
    unsafe {
        gl::DisableVertexArrayAttrib(vao, index);
    }
}
