use once_cell::sync::OnceCell;
use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_MODE: OnceCell<AtomicBool> = OnceCell::new();

/// Initializes the global debugging mode for OpenGL calls.
///
/// This function sets the `DEBUG_MODE` variable to `true` using an `AtomicBool`.
/// The function should be called at the start of your program to ensure
/// that the debugging mode is properly initialized before any GL calls are made.
///
/// # Panics
/// Panics if the `DEBUG_MODE` has already been initialized.
/// The `.unwrap()` ensures that this function is only called once.
///
/// # Example
/// ```no-run
/// init_debugging();
/// ```
///
/// This will enable GL error checking by default.
pub fn init_debugging() {
    DEBUG_MODE.set(AtomicBool::new(true)).unwrap();
}

/// Enables or disables OpenGL error checking at runtime.
///
/// This function allows you to toggle the GL debugging mode on or off.
/// It modifies the global `DEBUG_MODE` variable using an `AtomicBool`.
/// If the debugging mode has been initialized, the value of `DEBUG_MODE`
/// will be updated to match the provided `enabled` argument.
///
/// # Arguments
/// * `enabled` - A boolean indicating whether to enable (`true`) or disable (`false`)
///   GL error checking.
///
/// # Example
/// ```no-run
/// set_gl_debugging(true); // Enables GL error checking
/// set_gl_debugging(false); // Disables GL error checking
/// ```
///
/// If the debugging mode has not been initialized, this function will do nothing.
pub fn set_gl_debugging(enabled: bool) {
    if let Some(debug) = DEBUG_MODE.get() {
        debug.store(enabled, Ordering::Relaxed);
    }
}

/// Returns the current state of the OpenGL debugging mode.
///
/// This function checks whether GL error checking is currently enabled or disabled.
/// It reads the value of the `DEBUG_MODE` variable if it has been initialized, and
/// returns the current state of the debugging mode.
///
/// # Returns
/// * `true` - If GL error checking is enabled.
/// * `false` - If GL error checking is disabled or if the `DEBUG_MODE` has not been initialized.
///
/// # Example
/// ```no-run
/// if is_gl_debugging_enabled() {
///     println!("GL debugging is enabled.");
/// } else {
///     println!("GL debugging is disabled.");
/// }
/// ```
///
/// If the debugging mode has not been initialized, the function returns `false` by default.
fn is_gl_debugging_enabled() -> bool {
    DEBUG_MODE.get().map(|d| d.load(Ordering::Relaxed)).unwrap_or(false)
}
