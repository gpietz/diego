use std::time::Instant;

#[derive(Debug)]
pub struct DeltaTime {
    previous_time: Instant,
    actual: f64,
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self {
            previous_time: Instant::now(),
            actual: 0.0,
        }
    }
}

impl DeltaTime {
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
