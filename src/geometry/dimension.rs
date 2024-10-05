use std::fmt::{Display, Formatter};

/// A generic 2D dimension structure that represents the width and height of a 2D space.
///
/// This struct is generic over the type `T`, allowing it to represent dimensions
/// with various numeric types (e.g., `i32`, `f32`, `u32`).
///
/// # Fields
///
/// * `width` - The width of the 2D space, of type `T`.
/// * `height` - The height of the 2D space, of type `T`.
///
/// # Traits
///
/// The struct implements the following traits:
/// * `Debug` - Allows for formatting and printing the dimension.
/// * `Default` - Provides a default value where both width and height are set to their
///    default value of `T`.
/// * `Copy` and `Clone` - Enables easy copying and cloning of the struct.
///
/// # Example
///
/// ```no_run
/// let dimension: Dimension2D<i32> = Dimension2D { width: 1920, height: 1080 };
/// println!("Dimension: {}x{}", dimension.width, dimension.height);
///
/// let default_dimension: Dimension2D<f32> = Dimension2D::default();
/// println!("Default dimension: {}x{}", default_dimension.width, default_dimension.height);
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub struct Dimension2D<T> {
    pub width: T,
    pub height: T,
}

impl<T> Dimension2D<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<T: Display> Display for Dimension2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

/// A generic 3D dimension structure that represents the width, height, and depth of a 3D space.
///
/// This struct is generic over the type `T`, allowing it to represent dimensions
/// with various numeric types (e.g., `i32`, `f32`, `u32`).
///
/// # Fields
///
/// * `width` - The width of the 3D space, of type `T`.
/// * `height` - The height of the 3D space, of type `T`.
/// * `depth` - The depth of the 3D space, of type `T`.
///
/// # Traits
///
/// The struct implements the following traits:
/// * `Debug` - Allows for formatting and printing the dimension.
/// * `Default` - Provides a default value where width, height, and depth are set to their
///    default value of `T`.
/// * `Copy` and `Clone` - Enables easy copying and cloning of the struct.
///
/// # Example
///
/// ```no_run
/// let dimension: Dimension3D<i32> = Dimension3D { width: 1920, height: 1080, depth: 500 };
/// println!("3D Dimension: {}x{}x{}", dimension.width, dimension.height, dimension.depth);
///
/// let default_dimension: Dimension3D<f32> = Dimension3D::default();
/// println!("Default 3D Dimension: {}x{}x{}", default_dimension.width, default_dimension.height, default_dimension.depth);
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub struct Dimension3D<T> {
    pub width: T,
    pub height: T,
    pub depth: T,
}

impl<T> Dimension3D<T> {
    pub fn new(width: T, height: T, depth: T) -> Self {
        Self { width, height, depth }
    }
}

impl<T: Display> Display for Dimension3D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}x{}", self.width, self.height, self.depth)
    }
}
