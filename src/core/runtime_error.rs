use crate::core::runtime_info::RuntimeInfo;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Object creation failed: {0}")]
    ObjectCreationError(RuntimeInfo),
    #[error("Invalid buffer size: {0}")]
    InvalidBufferSize(RuntimeInfo),
    #[error("OpenGL error: {0}, error code: {1:?}")]
    OpenGLError(RuntimeInfo, Option<u32>),
}
