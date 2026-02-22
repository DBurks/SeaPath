pub mod calculations;
pub mod geodesy;
pub mod path_planning;
pub mod state;
pub mod units;

pub use geodesy::GeoPoint;
pub use state::StateVector;

// A simple test to make sure your Windows setup is working
#[cfg(test)]
mod tests {
    use super::units::*;

    #[test]
    fn test_knot_conversion() {
        // Updated to use the Speed struct from units.rs
        let mps = Speed::from_knots(1.0).mps();
        assert!((mps - 0.5144444444).abs() < 1e-6);
    }
}
