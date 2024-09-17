use std::cell::{RefCell};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::core::delta_time::DeltaTime;
use crate::core::event::Event;
use crate::display::window::WindowSettings;

pub trait ApplicationContext {
    fn events(&self) -> &[Event];
    fn delta_time(&self) -> f64;
    fn window_settings(&self) -> &WindowSettings;
    fn exit(&mut self);
    fn should_exit(&self) -> bool;
}

#[derive(Default)]
pub(crate) struct ApplicationContextImpl {
    pub delta_time: DeltaTime,
    pub should_exit: bool,
    pub window_settings: WindowSettings,
    pub events: Vec<Event>,
}

pub type SharedApplicationContext = Rc<RefCell<ApplicationContextImpl>>;

impl ApplicationContextImpl {
    pub fn new() -> SharedApplicationContext {
        Rc::new(RefCell::new(Self::default()))
    }
}

impl Debug for ApplicationContextImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("events: {}", self.events.len()))
    }
}

impl ApplicationContext for ApplicationContextImpl {
    fn events(&self) -> &[Event] {
        &self.events
    }

    fn delta_time(&self) -> f64 {
        self.delta_time.actual()
    }

    fn window_settings(&self) -> &WindowSettings {
        &self.window_settings
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }
    
    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
