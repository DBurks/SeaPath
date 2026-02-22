// seapath-core/src/units.rs

pub const NM_TO_METERS: f64 = 1852.0;
pub const KNOTS_TO_MPS: f64 = 0.5144444444;

#[derive(Debug, Clone, Copy)]
pub struct Angle(f64); // Stored as degrees internally

impl Angle {
    pub fn from_degrees(deg: f64) -> Self {
        Self(deg)
    }

    pub fn degrees(&self) -> f64 {
        self.0
    }

    pub fn radians(&self) -> f64 {
        self.0.to_radians()
    }

    pub fn from_radians(rad: f64) -> Self {
        Self(rad.to_degrees())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Distance(f64); // Stored as meters internally

impl Distance {
    pub fn from_nautical_miles(nm: f64) -> Self {
        Self(nm * NM_TO_METERS)
    }

    pub fn from_meters(m: f64) -> Self {
        Self(m)
    }

    pub fn nautical_miles(&self) -> f64 {
        self.0 / NM_TO_METERS
    }

    pub fn meters(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Speed(f64); // Stored as m/s internally

impl Speed {
    pub fn from_knots(kts: f64) -> Self {
        Self(kts * KNOTS_TO_MPS)
    }

    pub fn knots(&self) -> f64 {
        self.0 / KNOTS_TO_MPS
    }

    pub fn mps(&self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_conversions() {
        let a = Angle::from_degrees(180.0);
        assert!((a.radians() - std::f64::consts::PI).abs() < 1e-10);

        let b = Angle::from_radians(std::f64::consts::PI / 2.0);
        assert!((b.degrees() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_distance_conversions() {
        let d = Distance::from_nautical_miles(1.0);
        assert_eq!(d.meters(), 1852.0);

        let d2 = Distance::from_meters(1852.0);
        assert_eq!(d2.nautical_miles(), 1.0);
    }

    #[test]
    fn test_speed_conversions() {
        let s = Speed::from_knots(1.0);
        // Test internal m/s storage
        assert!((s.mps() - KNOTS_TO_MPS).abs() < 1e-10);

        // Test round-trip
        assert!((s.knots() - 1.0).abs() < 1e-10);
    }
}
