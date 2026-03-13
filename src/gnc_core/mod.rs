pub struct ShipState {
    pub lat: f64,
    pub lon: f64,
    pub heading: f32,
    pub speed: f32,
}

impl ShipState {
    pub fn new() -> Self {
        Self {
            lat: 0.0,
            lon: 0.0,
            heading: 0.0,
            speed: 0.0,
        }
    }
}