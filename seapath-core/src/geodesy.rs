// seapath-core/src/geodesy.rs
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoPoint {
    lat: f64,
    lon: f64,
}

impl GeoPoint {
    pub fn new(lat: f64, lon: f64) -> Result<Self, String> {
        // ... (validation logic stays the same)
        Ok(Self { lat, lon })
    }

    // Add 'pub' to these getters!
    pub fn lat(&self) -> f64 {
        self.lat
    }

    pub fn lon(&self) -> f64 {
        self.lon
    }

    pub fn to_radians(&self) -> (f64, f64) {
        (self.lat.to_radians(), self.lon.to_radians())
    }
}

pub struct Ellipsoid {
    pub semi_major_axis: f64,
    pub inverse_flattening: f64,
}

impl Ellipsoid {
    pub const WGS84: Ellipsoid = Ellipsoid {
        semi_major_axis: 6_378_137.0,
        inverse_flattening: 298.257_223_563,
    };

    pub fn flattening(&self) -> f64 {
        1.0 / self.inverse_flattening
    }

    pub fn semi_minor_axis(&self) -> f64 {
        self.semi_major_axis * (1.0 - self.flattening())
    }

    // Add 'pub' here!
    pub fn mean_radius(&self) -> f64 {
        (2.0 * self.semi_major_axis + self.semi_minor_axis()) / 3.0
    }
}

