use crate::gl::buffer::{bind_buffer, gen_buffers, buffer_data};
use crate::gl::types::{BufferType, BufferUsage, GlGetParameter};
use crate::glx::{Bindable, BindableState};
use crate::gl::state::get_integer_v;
use std::error::Error;

pub struct VertexBufferObject {
    vbo_id: u32,
    usage: BufferUsage,
}

// TODO Move id from constructor to upload_data  ...
impl VertexBufferObject {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let buffers = gen_buffers(1);
        if let Some(&vbo_id) = buffers?.first() {
            Ok(Self { vbo_id, usage: BufferUsage::StaticDraw })
        } else {
            Err("Failed to generate buffer ID".into())
        }
    }

    //TODO new_with_data() ....

    pub fn upload_data<T: AsRef<[u32]>>(&mut self, data: &T, usage: BufferUsage) -> Result<(), Box<dyn Error>> {
        
        

        let unbind_required = !self.is_bound()?;

        self.bind()?;
        self.usage = usage;
        buffer_data(BufferType::ArrayBuffer, data, usage);

        if unbind_required {
            self.unbind()?;
        }

        Ok(())
    }
}

impl Default for VertexBufferObject {
    fn default() -> Self {
        Self {
            vbo_id: 0,
            usage: BufferUsage::StaticDraw,
        }
        //Self::new().expect("Failed to generate VertexBufferObject")
    }
}

impl Bindable for VertexBufferObject {
    fn bind(&self) -> anyhow::Result<()> {
        bind_buffer(BufferType::ArrayBuffer, self.vbo_id);
        Ok(())
    }

    fn unbind(&self) -> anyhow::Result<()> {
        bind_buffer(BufferType::ArrayBuffer, 0);
        Ok(())
    }
}

impl BindableState for VertexBufferObject {
    fn is_bound(&self) -> anyhow::Result<bool> {
        let current_buffer_id = get_integer_v(GlGetParameter::ArrayBufferBinding);
        Ok(current_buffer_id as u32 == self.vbo_id)
    }
}
