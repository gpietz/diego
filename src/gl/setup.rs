use crate::gl::color::Color;
use crate::gl::types::Capability;

pub fn clear_color(color: Color) {
    unsafe {
        gl::ClearColor(color.r, color.g, color.b, color.a);
    }
}

pub fn view_port(x: f32, y: f32, width: f32, height: f32) {

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
