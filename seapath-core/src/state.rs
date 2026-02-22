use crate::geodesy::GeoPoint;
use crate::units::{Angle, Speed};

#[derive(Debug, Clone, Copy)]
pub struct StateVector {
    /// Microseconds since epoch (high precision for IMU integration)
    pub timestamp_us: u64,
    
    /// The physical location of the vessel
    pub position: GeoPoint,
    
    /// Magnetic or True North heading of the bow
    pub heading: Angle,
    
    /// Speed Over Ground (usually from GPS)
    pub sog: Speed,
    
    /// Course Over Ground (actual path over the seafloor)
    pub cog: Angle,
    
    /// Attitude: Tilt along the longitudinal axis
    pub roll: Angle,
    
    /// Attitude: Tilt along the lateral axis
    pub pitch: Angle,
    
    /// Rate of turn (Degrees per second) - Critical for Kalman Filters
    pub yaw_rate: f32, 
}

impl StateVector {
    /// Creates a "Zeroed" state at a specific location.
    /// Useful for initializing filters before sensor lock.
    pub fn new(pos: GeoPoint) -> Self {
        Self {
            timestamp_us: 0,
            position: pos,
            heading: Angle::from_degrees(0.0),
            sog: Speed::from_knots(0.0),
            cog: Angle::from_degrees(0.0),
            roll: Angle::from_degrees(0.0),
            pitch: Angle::from_degrees(0.0),
            yaw_rate: 0.0,
        }
    }

    /// Advances the ship's position based on current SOG and Heading.
    /// 'dt_s' is the delta time in seconds.
    pub fn advance_dead_reckoning(&mut self, dt_s: f64) {
        // 1. Calculate distance traveled in this time step
        let dist_meters = self.sog.mps() * dt_s;
        let distance = crate::units::Distance::from_meters(dist_meters);

        // 2. Calculate new position using the formula in calculations.rs
        let new_pos = crate::calculations::dead_reckon(
            self.position, 
            self.heading, 
            distance
        );

        // 3. Update the state
        self.position = new_pos;
        self.timestamp_us += (dt_s * 1_000_000.0) as u64;
    }
}