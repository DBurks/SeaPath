// seapath-core/src/geodesy.rs

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoPoint {
    lat: f64,
    lon: f64,
}

impl GeoPoint {
    pub fn new(lat: f64, lon: f64) -> Result<Self, String> {
        if !(-90.0..=90.0).contains(&lat) {
            return Err(format!("Latitude {} is out of range (-90 to 90)", lat));
        }
        if !(-180.0..=180.0).contains(&lon) {
            return Err(format!("Longitude {} is out of range (-180 to 180)", lon));
        }
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

    /// Calculates the "Mercator Latitude" (Meridional Parts).
    /// This is the vertical coordinate on a flat Mercator map.
    pub fn mercator_latitude(&self) -> f64 {
        let lat_rad = self.lat.to_radians();

        // Clamp latitude to +/- 89.5 degrees to avoid the infinity asymptote at the poles
        let limit = 89.5f64.to_radians();
        let clamped_lat = lat_rad.clamp(-limit, limit);

        // The Bowditch formula: ln(tan(45° + φ/2))
        (std::f64::consts::FRAC_PI_4 + (clamped_lat / 2.0))
            .tan()
            .ln()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geopoint_creation() {
        let point = GeoPoint::new(45.0, -75.0);
        assert!(point.is_ok());
        let p = point.unwrap();
        assert_eq!(p.lat(), 45.0);
        assert_eq!(p.lon(), -75.0);
    }

    #[test]
    fn test_invalid_geopoint() {
        let point = GeoPoint::new(95.0, 0.0);
        assert!(point.is_err());
    }

    #[test]
    fn test_mean_radius_wgs84() {
        let radius = Ellipsoid::WGS84.mean_radius();
        // WGS84 mean radius is approximately 6,371,008 meters
        assert!((radius - 6371008.0).abs() < 1.0);
    }
}
