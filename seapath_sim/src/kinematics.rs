pub struct VesselKinematics {
    // Earth-Fixed Positions
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub heading_rad: f32,

    // Body-Fixed Velocities
    pub surge_knots: f32,   // Forward velocity (u)
    pub sway_knots: f32,    // Lateral velocity (v)
    pub yaw_rate_rads: f32, // Angular velocity (r)

    // Constants for coordinate conversion (meters per degree approximately)
    lat_meters_per_deg: f64,
    lon_meters_per_deg: f64,
}

impl VesselKinematics {
    pub fn new(start_lat: f64, start_lon: f64, start_heading: f32) -> Self {
        Self {
            latitude_deg: start_lat,
            longitude_deg: start_lon,
            heading_rad: start_heading,
            surge_knots: 10.0, // Constant forward cruising speed for now
            sway_knots: 0.0,
            yaw_rate_rads: 0.0,
            // Approximations centered around mid-latitudes
            lat_meters_per_deg: 111132.0,
            lon_meters_per_deg: 85000.0,
        }
    }

    /// Steps the kinematic equations forward by dt seconds based on current actuator demands
    pub fn update(&mut self, rudder_deg: f32, dt: f32) {
        // 1. SIMPLE ACTUATOR MODEL: Rudder angle directly induces a proportional yaw rate
        // (We will add true hydrodynamic coefficients and momentum next, but this hooks up the loop)
        let rudder_rad = rudder_deg.to_radians();

        // Let's assume a basic steering gain where rudder induces an angular velocity change
        self.yaw_rate_rads = -0.05 * rudder_rad;

        // 2. COORDINATE TRANSFORMATION (Body Frame -> Earth Frame)
        // Convert speed from knots to meters per second (1 knot = 0.514444 m/s)
        let u_ms = self.surge_knots * 0.514444;
        let v_ms = self.sway_knots * 0.514444;

        let cos_psi = self.heading_rad.cos();
        let sin_psi = self.heading_rad.sin();

        // Calculate velocity vector in local tangent plane meters/sec
        let x_dot = (u_ms * cos_psi) - (v_ms * sin_psi);
        let y_dot = (u_ms * sin_psi) + (v_ms * cos_psi);
        let psi_dot = self.yaw_rate_rads;

        // 3. EULER INTEGRATION
        let delta_x = x_dot * dt;
        let delta_y = y_dot * dt;

        // Update Geodetic Coordinates
        self.latitude_deg += (delta_x as f64) / self.lat_meters_per_deg;
        self.longitude_deg += (delta_y as f64) / self.lon_meters_per_deg;

        // Update Heading and wrap to [0, 2*PI]
        self.heading_rad += psi_dot * dt;
        if self.heading_rad < 0.0 {
            self.heading_rad += 2.0 * std::f32::consts::PI;
        } else if self.heading_rad > 2.0 * std::f32::consts::PI {
            self.heading_rad -= 2.0 * std::f32::consts::PI;
        }
    }
}
