use crate::geodesy::{GeoPoint, Ellipsoid};
use crate::units::{Distance, Angle};

/// Calculates the great circle distance between two GeoPoints using the Haversine formula
pub fn great_circle_distance(p1: GeoPoint, p2: GeoPoint) -> Distance {
    let (phi1, _) = p1.to_radians();
    let (phi2, _) = p2.to_radians();
    
    // Using the public getters .lat() and .lon()
    let delta_phi = (p2.lat() - p1.lat()).to_radians();
    let delta_lon = (p2.lon() - p1.lon()).to_radians();

    let a = (delta_phi / 2.0).sin().powi(2)
        + phi1.cos() * phi2.cos() * (delta_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    // Now mean_radius() is public and accessible
    let distance_meters = Ellipsoid::WGS84.mean_radius() * c;

    Distance::from_meters(distance_meters)
}

/// Calculates the initial great circle bearing from point p1 to p2
pub fn great_circle_initial_bearing(p1: GeoPoint, p2: GeoPoint) -> Angle {
    let (phi1, _) = p1.to_radians();
    let (phi2, _) = p2.to_radians();
    let delta_lon = (p2.lon() - p1.lon()).to_radians();

    let y = delta_lon.sin() * phi2.cos();
    let x = phi1.cos() * phi2.sin() - phi1.sin() * phi2.cos() * delta_lon.cos();
    
    let bearing_rad = y.atan2(x);
    let bearing_deg = (bearing_rad.to_degrees() + 360.0) % 360.0;
    
    Angle::from_degrees(bearing_deg)
}

/// Calculates the Rhumb Line distance and bearing (Mercator Sailing)
/// As defined in Bowditch / Pub. 229
pub fn rhumb_line_navigation(p1: GeoPoint, p2: GeoPoint) -> (Distance, Angle) {
    let (phi1, lam1) = p1.to_radians();
    let (phi2, lam2) = p2.to_radians();
    
    let d_phi = phi2 - phi1;
    let mut d_lam = lam2 - lam1;

    // Handle the Date Line (180th meridian)
    if d_lam.abs() > std::f64::consts::PI {
        d_lam = if d_lam > 0.0 { 
            d_lam - 2.0 * std::f64::consts::PI 
        } else { 
            d_lam + 2.0 * std::f64::consts::PI 
        };
    }

    // Mercator "stretching" factor (Difference in Meridional Parts)
    let d_psi = ((phi2 / 2.0 + std::f64::consts::FRAC_PI_4).tan() / 
                 (phi1 / 2.0 + std::f64::consts::FRAC_PI_4).tan()).ln();

    // Bearing (Course)
    let bearing_rad = d_lam.atan2(d_psi);
    
    // Distance
    let q = if d_phi.abs() > 1e-9 { d_phi / d_psi } else { phi1.cos() };
    let distance_meters = (d_phi.powi(2) + (q * d_lam).powi(2)).sqrt() * Ellipsoid::WGS84.semi_major_axis;

    (
        Distance::from_meters(distance_meters),
        Angle::from_degrees((bearing_rad.to_degrees() + 360.0) % 360.0)
    )
}

/// Predicts a new position based on a starting point, bearing, and distance.
pub fn dead_reckon(start: GeoPoint, bearing: Angle, distance: Distance) -> GeoPoint {
    let (phi1, lam1) = start.to_radians();
    let brng = bearing.radians();
    
    // Angular distance (distance / earth radius)
    let d_r = distance.meters() / Ellipsoid::WGS84.mean_radius();

    let phi2 = (phi1.sin() * d_r.cos() + 
                phi1.cos() * d_r.sin() * brng.cos()).asin();
    
    let lam2 = lam1 + (brng.sin() * d_r.sin() * phi1.cos())
                .atan2(d_r.cos() - phi1.sin() * phi2.sin());

    // Convert back to degrees and return a new point
    GeoPoint::new(phi2.to_degrees(), lam2.to_degrees()).unwrap()
}