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