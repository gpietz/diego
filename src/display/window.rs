use crate::display::types::Size;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, ContextWrapper, GlRequest, PossiblyCurrent};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use glutin::dpi::LogicalSize;

const DEFAULT_WINDOW_TITLE: &str = "DIEGO";
const DEFAULT_WINDOW_WIDTH: u32 = 800;
const DEFAULT_WINDOW_HEIGHT: u32 = 600;
const OPENGL_API_VERSION: GlRequest = GlRequest::Specific(Api::OpenGl, (4, 5));

pub trait Window {
    fn show(&self);
    fn close(&self);
    fn center(&self);
}

pub trait GLWindow : Window {
    fn swap_buffers(&mut self);
}

#[derive(Debug, Clone)]
pub struct WindowSettings {
    title: String,
    size: Size,
    vsync: bool,
    resizable: bool,
    visible: bool,
    exit_on_esc: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            title: DEFAULT_WINDOW_TITLE.to_string(),
            size: Size::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
            vsync: false,
            resizable: false,
            visible: true,
            exit_on_esc: true,
        }
    }
}

impl WindowSettings {
    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_size<T: Into<Size>>(mut self, size: T) -> Self {
        self.size = size.into();
        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
    
    pub fn with_exit_on_esc(mut self, exit_on_esc: bool) -> Self {
        self.exit_on_esc = exit_on_esc;
        self
    }

    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn get_resizable(&self) -> bool {
        self.resizable
    }

    pub fn get_visible(&self) -> bool {
        self.visible
    }
    
    pub fn get_exit_on_esc(&self) -> bool {
        self.exit_on_esc
    }
}

pub(crate) struct WinitWindow {
    context:  ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    close_requested: Arc<AtomicBool>,
}

impl WinitWindow {
    pub(crate) fn create(settings: WindowSettings, event_loop: &EventLoop<()>) -> Self {

        // winit 0.30.5 stuff *not working*
        // ==================
        //
        // let event_loop = EventLoop::new().unwrap();
        //
        // let size = LogicalSize::new(settings.size.width, settings.size.height);
        // let mut window_attributes = WindowAttributes::default()
        //     .with_title(settings.get_title())
        //     .with_inner_size(size);
        //
        // window_attributes.resizable = settings.get_resizable();
        // window_attributes.visible = false;
        //
        // let template_builder = glutin::config::ConfigTemplateBuilder::new();
        //
        // let x = glutin::DisplayBuilder::new().with_window_attributes(Some(window_attributes)).build(&event_loop);
        // event_loop.create_window(window_attributes).unwrap()

        let window_size = LogicalSize::new(settings.size.width, settings.size.height);
        let window_builder = WindowBuilder::new()
            .with_title(settings.get_title())
            .with_inner_size(window_size)
            .with_resizable(settings.resizable)
            .with_visible(false);

        let windowed_context = ContextBuilder::new()
            .with_double_buffer(Some(true))
            .with_gl(OPENGL_API_VERSION)
            .with_vsync(settings.vsync)
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        // Make the context current
        let windowed_context : ContextWrapper<PossiblyCurrent, glutin::window::Window> =
            unsafe { windowed_context.make_current().unwrap() };

        // Load the OpenGL function pointers using the window's current context.
        gl::load_with(|s| windowed_context.get_proc_address(s));

        WinitWindow {
            context: windowed_context, close_requested: Arc::new(AtomicBool::new(false))
        }
    }
}

impl Window for WinitWindow {
    fn show(&self) {
        let window = self.context.window();
        window.set_visible(true);
    }

    fn close(&self) {
        // The close method must trigger a custom event.
        // Since the event loop is running in a blocking manner, you could use a variable or a custom event
        // to signal a close operation. However, here you can only simulate the CloseRequested event.
        self.close_requested.store(true, Ordering::Relaxed);
    }

    fn center(&self) {
        todo!()
    }
}

impl GLWindow for WinitWindow {
    fn swap_buffers(&mut self) {
        self.context.swap_buffers().unwrap();
    }
}
