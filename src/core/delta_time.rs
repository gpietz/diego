use std::time::Instant;

/// A structure for tracking and calculating time deltas between frames or updates.
///
/// The `DeltaTime` struct stores the time of the last update (`previous_time`) and the
/// duration of the most recent update (`actual`), which is measured in seconds.
///
/// # Fields
///
/// * `previous_time` - An `Instant` representing the time of the previous update.
/// * `actual` - A `f64` value representing the time in seconds that has passed since
///   the last update.
///
/// # Example
///
/// ```no_run
/// let mut delta_time = DeltaTime {
///     previous_time: Instant::now(),
///     actual: 0.0,
/// };
///
/// ```
///
/// Call delta_time.update() to calculate time deltas during your game loop or application update.
#[derive(Debug)]
pub struct DeltaTime {
    previous_time: Instant,
    actual: f64,
}

impl Default for DeltaTime {
    /// Creates a default instance of the `DeltaTime` struct.
    ///
    /// This function initializes `previous_time` to the current instant (`Instant::now()`) 
    /// and sets `actual` to `0.0`. It is typically used when creating a `DeltaTime` 
    /// instance without requiring custom initialization values.
    fn default() -> Self {
        Self {
            previous_time: Instant::now(),
            actual: 0.0,
        }
    }
}

impl DeltaTime {
    // Updates the time tracking and returns the elapsed time since the last update.
    ///
    /// This function calculates the time difference between the current time and the
    /// previously stored time (`previous_time`). It updates the `actual` field with
    /// the new duration (in seconds) and sets `previous_time` to the current time.
    ///
    /// # Returns
    ///
    /// A `f64` value representing the time elapsed (in seconds) since the last call to `update`.
    ///
    /// # Example
    /// ```
    /// let elapsed_time = delta_time.update();
    /// println!("Elapsed time: {} seconds", elapsed_time);
    /// ```
    pub fn update(&mut self) -> f64 {
        let current_time = Instant::now();
        self.actual = current_time.duration_since(self.previous_time).as_secs_f64();
        self.previous_time = current_time;
        self.actual
    }

    pub fn actual(&self) -> f64 {
        self.actual
    }
}

impl Into<f64> for DeltaTime { 
    fn into(self) -> f64 {
        self.actual
    }
}
