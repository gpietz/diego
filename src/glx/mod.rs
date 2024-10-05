use anyhow::Result;

pub mod vertex_layout;
pub mod vertex_attribute;
pub mod vertex_data_type;
pub mod vertex_attribute_type;
pub mod vertex_utils;
pub mod vertex_layout_manager;
pub mod vertex_array_object;
mod vertex_buffer_objects;

/// A trait for objects that can be bound and unbound in the context of OpenGL 
/// or similar graphics APIs.
///
/// Implement this trait for types that need to be bound to a context 
/// (e.g., buffers, textures, VAOs) and subsequently unbound after use.
///
/// # Methods
/// - `bind`: Binds the object to the current context. 
///    Returns a `Result` indicating success or failure.
/// - `unbind`: Unbinds the object from the current context. 
///    Returns a `Result` indicating success or failure.
pub trait Bindable {
    fn bind(&self) -> Result<()>;
    fn unbind(&self) -> Result<()>;
}

/// A trait for checking the binding state of objects in the context of OpenGL or
/// similar graphics APIs.
///
/// Implement this trait for types that need to provide information about their binding state, 
/// indicating whether the object is currently bound to the context (e.g., buffers, textures, VAOs).
///
/// # Methods
/// - `is_bound`: Returns a `Result<bool>` indicating whether the object is currently bound (`true`) 
///               or not (`false`).
pub trait BindableState {
    fn is_bound(&self) -> Result<bool>;
}
