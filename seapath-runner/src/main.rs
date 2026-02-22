use seapath_core::geodesy::GeoPoint;
use seapath_core::state::StateVector;
use seapath_core::units::{Angle, Speed};

fn main() {
    let start_pos = GeoPoint::new(39.8683, -104.9719).unwrap();
    let mut ship = StateVector::new(start_pos);
    
    ship.sog = Speed::from_knots(20.0); // Fast cruiser
    ship.heading = Angle::from_degrees(45.0); // Steering North-East

    println!("Starting Sea Trial...");
    println!("Initial Pos: Lat {:.4}, Lon {:.4}", ship.position.lat(), ship.position.lon());

    // Simulate 1 hour of travel in 10-minute increments
    for min in (10..=60).step_by(10) {
        ship.advance_dead_reckoning(600.0); // 600 seconds = 10 minutes
        println!(
            "T + {} min: Lat {:.4}, Lon {:.4}", 
            min, ship.position.lat(), ship.position.lon()
        );
    }
}