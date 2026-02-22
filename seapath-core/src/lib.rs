pub mod units;
pub mod geodesy;
pub mod state;
pub mod calculations;

pub use state::StateVector;
pub use geodesy::GeoPoint;

// A simple test to make sure your Windows setup is working
#[cfg(test)]
mod tests {
    use super::units::*;

    #[test]
    fn test_knot_conversion() {
        let mps = knots_to_ms(1.0);
        assert!((mps - 0.514444).abs() < 1e-6);
    }
}