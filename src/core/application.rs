use crate::core::application_context::{ApplicationContextImpl, SharedApplicationContext};
use crate::core::main_loop::{DefaultMainLoop, SharedApplicationMainLoop};
use crate::display::window::{GLWindow, Window, WindowSettings, WinitWindow};
use crate::events::event::Event;
use crate::events::event::Event::WindowCloseRequested;
use crate::gl::color::Color;
use crate::gl::rendering::clear;
use crate::gl::setup::clear_color;
use glutin::event::KeyboardInput;
use glutin::event::VirtualKeyCode;
use glutin::event_loop::{ControlFlow, EventLoop};

pub struct Application {
    main_loop: SharedApplicationMainLoop,
    running: bool,
    application_context: SharedApplicationContext,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            main_loop: DefaultMainLoop::new(),
            running: false,
            application_context: ApplicationContextImpl::new(),
        }
    }
}

impl Application {
    pub fn with_main_loop(mut self, main_loop: SharedApplicationMainLoop) -> Self {
        if self.running {
            panic!("Application already running; main loop can't be changed!");
        }
        self.main_loop = main_loop;
        self
    }

    pub fn with_window_settings(self, window_settings: WindowSettings) -> Self {
        self.application_context.borrow_mut().window_settings = window_settings;
        self
    }

    #[allow(unreachable_code)]
    pub fn run(&mut self) {
        let context = self.application_context.clone();
        self.running = true;

        let main_loop_clone = self.main_loop.clone();

        // Create window
        let event_loop = EventLoop::new();
        let window_settings = self.application_context.borrow().window_settings.clone();
        let mut window = WinitWindow::create(window_settings.clone(), &event_loop);
        prepare_window(&mut window, &window_settings);

        // Start event loop
        let mut event_collection  = Vec::<Event>::new();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = glutin::event_loop::ControlFlow::Poll;

            // Update delta time
            context.borrow_mut().delta_time.update();

            // Handle events
            handle_event(&mut event_collection, &event, control_flow, &context);

            //main_loop_clone.borrow_mut().on_update(delta_time);

            clear_color(Color::BLACK);
            clear();
            window.swap_buffers();

            if context.borrow().should_exit {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
        });

        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn exit(&mut self) {
        self.application_context.borrow_mut().should_exit = true;
    }
}

fn prepare_window(window: &mut WinitWindow, settings: &WindowSettings) {
    // Clear window background with black background
    clear_color(Color::BLACK);
    clear();
    window.swap_buffers();

    // Show window  
    if settings.get_visible() {
        window.show();
    }
}

fn handle_event(event_collection: &mut Vec<Event>,
                event: &glutin::event::Event<()>,
                control_flow: &ControlFlow,
                context: &SharedApplicationContext) {
    match event {
        glutin::event::Event::WindowEvent { event, .. } =>
            handle_window_event(event_collection, event, control_flow, context),
        _ => {}
    }
}

fn handle_window_event(event_collection: &mut Vec<Event>,
                       event: &glutin::event::WindowEvent,
                       control_flow: &ControlFlow,
                       context: &SharedApplicationContext) {
    match event {
        glutin::event::WindowEvent::CloseRequested => {
            handle_window_close(event_collection, context);
        }
        glutin::event::WindowEvent::KeyboardInput { input, .. } => {
            handle_keyboard_input(event_collection, input, control_flow, context);
        }
        _ => {}
    }
}

fn handle_window_close(event_collection: &mut Vec<Event>,
                       context: &SharedApplicationContext) {
    event_collection.push(WindowCloseRequested);
    context.borrow_mut().should_exit = true;
}

fn handle_keyboard_input(_event_collection: &mut Vec<Event>,
                    input: &KeyboardInput,
                    _control_flow: &ControlFlow,
                    context: &SharedApplicationContext) {
    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
        if context.borrow().window_settings.get_exit_on_esc() {
            context.borrow_mut().should_exit = true;
        }
    }
}
