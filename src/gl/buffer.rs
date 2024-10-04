use crate::core::runtime_error::RuntimeError;
use crate::core::runtime_info::RuntimeInfo;
use crate::create_runtime_info;
use crate::gl::types::{BufferType, BufferUsage};
use crate::gl::GLConstant;
use ogl::types::{GLsizei, GLsizeiptr, GLuint, GLvoid};

/// Generates a specified number of buffer IDs and returns them as a vector.
/// # Arguments
/// * `count` - The number of buffer IDs to generate.
/// # Returns
/// A vector containing the generated buffer IDs.
pub fn gen_buffers(count: u32) -> Result<Vec<u32>, RuntimeError> {
    let mut buffers: Vec<u32> = vec![0; count as usize];
    unsafe {
        gl::GenBuffers(count as GLsizei, buffers.as_mut_ptr());
    }

    // Check whether the buffer IDs are correct
    if buffers.iter().any(|&id| id == 0) {
        unsafe {
            let error_code = gl::GetError();
            if error_code != gl::NO_ERROR {
                let err_msg = format!("OpenGL error code: {}", error_code);
                //error!("{}", err_msg);  
                return Err(RuntimeError::OpenGLError(create_runtime_info!(&err_msg), Some(error_code)));
            }
        }
        return Err(RuntimeError::ObjectCreationError(create_runtime_info!(
            "Failed to generate buffer objects"
        )));
    }

    Ok(buffers)
}

/// Binds a buffer to a specified target in OpenGL.
/// # Arguments
/// * `target` - The target to which the buffer should be bound (e.g., `ARRAY_BUFFER`).
/// * `buffer_id` - The ID of the buffer to bind.
pub fn bind_buffer(target: BufferType, buffer_id: u32) {
    unsafe {
        gl::BindBuffer(target.to_gl_constant(), buffer_id);
    }
}

/// Uploads data to a specified buffer in OpenGL.
/// Ensure that the buffer is properly bound before calling this function.
/// # Arguments
/// * `target` - The buffer target (e.g., `ARRAY_BUFFER`).
/// * `dat` - A slice or vector containing the data to upload.
/// * `usage` - The intended usage pattern of the buffer (e.g., `STATIC_DRAW`).
pub fn buffer_data<T, D>(target: BufferType, dat: D, usage: BufferUsage)
where
    D: AsRef<[T]>,
{
    let slice = dat.as_ref();
    let size = (slice.len() * size_of::<T>()) as GLsizeiptr;
    unsafe {
        gl::BufferData(
            target.to_gl_constant(),
            size,
            slice.as_ptr() as *const GLvoid,
            usage.to_gl_constant()
        )
    }
}

/// Deletes the specified buffers from OpenGL.
/// # Arguments
/// * `buffers` - A slice or vector of buffer IDs to delete.
pub fn delete_buffers<T: AsRef<[u32]>>(buffers: T) {
    let slice = buffers.as_ref();
    unsafe {
        gl::DeleteBuffers(slice.len() as GLsizei, slice.as_ptr());
    }
}

/// Checks if a buffer ID represents a valid buffer in OpenGL.
/// # Arguments
/// * `buffer_id` - The buffer ID to check.
/// # Returns
/// `true` if the buffer ID is valid, `false` otherwise.
pub fn is_buffer(buffer_id: u32) -> bool {
    unsafe {
        gl::IsBuffer(buffer_id as GLuint) > 0
    }
}
