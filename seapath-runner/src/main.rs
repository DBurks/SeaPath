use seapath_core::geodesy::GeoPoint;
use seapath_core::path_planning::{Leg, Waypoint};

fn main() {
    // 1. Define our Waypoints
    let nyc_wp = Waypoint::new("NYC", 40.6413, -73.7781);
    let ldn_wp = Waypoint::new("London", 51.4700, -0.4543);

    // 2. Create the active Leg (This calculates the Desired Track automatically)
    let leg = Leg::new(nyc_wp, ldn_wp);
    println!(
        "Leg from NYC to London. Desired Track: {:.2}°",
        leg.desired_track.degrees()
    );

    // 3. Simulate a "Drifting" Submarine
    // Let's put the sub slightly South of the track (lower latitude)
    let current_pos = GeoPoint::new(38.0, -60.0).unwrap();

    // 4. Calculate the Leg Progress (XTE and ATD)
    let progress = leg.get_progress(&current_pos);

    println!("--- Navigation Status ---");
    println!(
        "Progress Along Track: {:.2} km",
        progress.atd.meters() / 1000.0
    );
    println!(
        "Cross-Track Deviation: {:.2} km",
        progress.xte.meters() / 1000.0
    );

    if progress.xte.meters().abs() < 500.0 {
        println!("Status: ON TRACK");
    } else if progress.xte.meters() > 0.0 {
        println!("Status: STEER LEFT (Drifted Right)");
    } else {
        println!("Status: STEER RIGHT (Drifted Left)");
    }
    println!("Distance to Go: {:.2} km", progress.dtg.meters() / 1000.0);
    println!(
        "Total Leg Length: {:.2} km",
        leg.total_distance.meters() / 1000.0
    );
}
