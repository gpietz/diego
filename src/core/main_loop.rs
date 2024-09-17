use std::cell::RefCell;
use std::rc::Rc;

/// The `ApplicationMainLoop` trait defines the behavior for the main loop of an application.
///
/// # Overview
/// Implementing this trait allows an application to have a custom logic that
/// is executed repeatedly within the main loop. The main purpose of this trait
/// is to update the state of the application in each iteration of the loop,
/// with the time between frames being provided as `delta_time`.
///
/// # Methods
///
/// - `on_update(&mut self, delta_time: f64)`
///     - This method is called on each iteration of the main loop.
///     - It receives a `delta_time` parameter, which represents the elapsed
///       time (in seconds) since the last update. This allows for frame-independent
///       logic (e.g., animations, physics).
///
/// # Example
///
/// ```rust
/// use diego::core::main_loop::ApplicationMainLoop;
///
/// struct MyMainLoop;
///
/// impl ApplicationMainLoop for MyMainLoop {
///     fn on_update(&mut self, delta_time: f64) {
///         // Custom logic for each frame
///         println!("Updating the application, delta time: {}", delta_time);
///     }
/// }
/// ```
pub trait ApplicationMainLoop {
    fn on_update(&mut self, delta_time: f64);
}

/// A shared, mutable reference to an `ApplicationMainLoop` implementation.
///
/// # Overview
/// `SharedApplicationMainLoop` is a type alias for a `Rc<RefCell<dyn ApplicationMainLoop>>`,
/// which provides a way to share a reference to an `ApplicationMainLoop` implementation
/// across multiple owners, while allowing internal mutability.
///
/// This is useful when multiple components need access to the same instance of an
/// `ApplicationMainLoop` and need to mutate its state during the execution of the application.
///
/// - `Rc` is a reference-counted pointer that allows shared ownership of the data.
/// - `RefCell` provides interior mutability, allowing mutation of the data even though it is shared.
/// - `dyn ApplicationMainLoop` indicates that the stored object is any type that implements the
///   `ApplicationMainLoop` trait.
///
/// # Example
///
/// ```rust
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// use diego::core::main_loop::ApplicationMainLoop;
///
/// struct MyMainLoop;
///
/// impl ApplicationMainLoop for MyMainLoop {
///     fn on_update(&mut self, delta_time: f64) {
///         println!("Updating main loop with delta time: {}", delta_time);
///     }
/// }
///
/// let main_loop: SharedApplicationMainLoop = Rc::new(RefCell::new(MyMainLoop));
///
/// main_loop.borrow_mut().on_update(0.16);
/// ```
///
/// # Advantages
/// This type alias allows for flexible sharing and mutation of the `ApplicationMainLoop`
/// without worrying about lifetime complexities or borrowing rules.
pub type SharedApplicationMainLoop = Rc<RefCell<dyn ApplicationMainLoop>>;

// Standard empty mainloop
pub struct DefaultMainLoop;

impl DefaultMainLoop {
    pub(crate) fn new() -> SharedApplicationMainLoop {
        Rc::new(RefCell::new(DefaultMainLoop {}))
    }
}

impl ApplicationMainLoop for DefaultMainLoop {
    fn on_update(&mut self, _delta_time: f64) {}
}
