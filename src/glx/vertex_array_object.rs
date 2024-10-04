use std::fmt::{Display, Formatter};
use crate::gl::state::get_integer_v;
use crate::gl::types::GlGetParameter;
use crate::gl::vao::{bind_vertex_array, gen_vertex_array};
use crate::glx::{Bindable, BindableState};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct VertexArrayObject {
    id: u32,
    name: Option<String>,
}

impl Default for VertexArrayObject {
    fn default() -> Self {
        Self {
            id: gen_vertex_array(),
            name: None,
        }
    }
}

impl Display for VertexArrayObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(name) => write!(f, "VAO: {}", name),
            None => write!(f, "VAO"),
        }
    }
}

impl VertexArrayObject {
    fn with_name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl Bindable for VertexArrayObject {
    fn bind(&self) -> anyhow::Result<()> {
        bind_vertex_array(self.id);
        Ok(())
    }

    fn unbind(&self) -> anyhow::Result<()> {
        bind_vertex_array(0);
        Ok(())
    }
}

impl BindableState for VertexArrayObject {
    fn is_bound(&self) -> anyhow::Result<bool> {
        Ok(self.id > 0 && self.id == get_integer_v(GlGetParameter::VertexArrayBinding) as u32)   
    }
}
