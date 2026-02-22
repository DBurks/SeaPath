use seapath_core::geodesy::GeoPoint;
use seapath_core::path_planning::{Leg, Waypoint};

#[test]
fn test_basic_navigation_flow() {
    let start_point = GeoPoint::new(0.0, 0.0).unwrap();
    let end_point = GeoPoint::new(0.1, 0.0).unwrap(); // Northward

    let start_wp = Waypoint {
        name: "Origin".to_string(),
        location: start_point,
    };
    let end_wp = Waypoint {
        name: "North_Point".to_string(),
        location: end_point,
    };

    let leg = Leg::new(start_wp, end_wp);

    // Simulate being at the start
    let progress = leg.get_progress(&start_point);

    assert!(progress.dtg.meters() > 0.0);
    assert!(progress.atd.meters() < 1.0); // Basically at the start
}
