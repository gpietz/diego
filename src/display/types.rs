/// Represents the size of a window or display in terms of its dimensions.
///
/// The `Size` struct holds the width and height of a window or display in pixels.
///
/// # Examples
///
/// ```no-run
/// use diego::display::types::Size;
///
/// let window_size = Size { width: 1920, height: 1080 };
/// println!("Window size: {}x{}", window_size.width, window_size.height);
/// ```
///
/// # Fields
///
/// * `width`: The width of the window in pixels.
/// * `height`: The height of the window in pixels.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    /// The width of the display in pixels.
    pub width: u32,
    /// The height of the display in pixels.
    pub height: u32,
}

impl Size {
    /// Creates a new `Size` struct with the given width and height.
    /// # Parameters
    /// * `width`: The width of the window or display in pixels.
    /// * `height`: The height of the window or display in pixels.
    pub fn new(width: u32, height: u32) -> Size {
        Size { width, height }
    }
}

impl From<[u32; 2]> for Size {
    fn from(value: [u32; 2]) -> Size {
        Size { width: value[0], height: value[1] }
    }
}

impl From<(u32, u32)> for Size {
    fn from(value: (u32, u32)) -> Size {
        Size { width: value.0, height: value.1 }
    }
}

impl From<Size> for [u32; 2] {
    fn from(value: Size) -> [u32; 2] {
        [value.width, value.height]
    }
}

impl From<Size> for (u32, u32) {
    fn from(value: Size) -> ( u32, u32 ) {
        (value.width, value.height)
    }
}

/// Represents the position of a window on the screen.
///
/// The `Position` struct holds the coordinates of the top-left corner of a window.
/// The position is defined using `x` and `y` values, where `x` represents the horizontal
/// distance from the left edge of the screen, and `y` represents the vertical distance
/// from the top edge of the screen.
///
/// # Fields
///
/// * `x`: The horizontal position of the window (distance from the left of the screen).
/// * `y`: The vertical position of the window (distance from the top of the screen).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    /// The horizontal position of the window.
    pub x: i32,
    /// The vertical position of the window.
    pub y: i32,
}

impl Position {
    /// Creates a new `Position` struct with the given `x` and `y` coordinates.
    /// # Parameters
    /// * `x`: The horizontal position of the window (distance from the left of the screen).
    /// * `y`: The vertical position of the window (distance from the top of the screen).
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

impl From<[i32; 2]> for Position {
    fn from(value: [i32; 2]) -> Position {
        Position { x: value[0], y: value[1] }
    }
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Position {
        Position { x: value.0, y: value.1 }
    }
}

impl From<Position> for [i32; 2] {
    fn from(value: Position) -> [i32; 2] {
        [value.x, value.y]
    }
}

impl From<Position> for (i32, i32) {
    fn from(value: Position) -> ( i32, i32 ) {
        (value.x, value.y)
    }
}
