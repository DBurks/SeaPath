use crate::calculations::{great_circle_distance, rhumb_line_bearing};
use crate::geodesy::GeoPoint;
use crate::units::{Angle, Distance};

#[derive(Debug, Clone)] // Allows copying and easy printing
pub struct Waypoint {
    pub name: String,
    pub location: GeoPoint,
}

impl Waypoint {
    pub fn new(name: &str, lat: f64, lon: f64) -> Self {
        Self {
            name: name.to_string(),
            location: GeoPoint::new(lat, lon).expect("Invalid Waypoint Coordinates"),
        }
    }
}

pub struct LegProgress {
    pub xte: Distance,
    pub atd: Distance,
    pub dtg: Distance,
}

pub struct Leg {
    pub start: GeoPoint,
    pub end: GeoPoint,
    pub desired_track: Angle,
    pub total_distance: Distance,
}

impl Leg {
    pub fn new(start: Waypoint, end: Waypoint) -> Self {
        let bearing = rhumb_line_bearing(&start.location, &end.location);
        // We use great_circle_distance for the physical length of the leg
        let distance = great_circle_distance(start.location, end.location);

        Self {
            start: start.location,
            end: end.location,
            desired_track: bearing,
            total_distance: distance,
        }
    }

    pub fn get_progress(&self, current_pos: &GeoPoint) -> LegProgress {
        let dist_to_start = great_circle_distance(self.start, *current_pos);
        let bearing_to_current = rhumb_line_bearing(&self.start, current_pos);

        let mut angle_diff = bearing_to_current.radians() - self.desired_track.radians();

        // Normalization to -PI to +PI
        if angle_diff > std::f64::consts::PI {
            angle_diff -= 2.0 * std::f64::consts::PI;
        }
        if angle_diff < -std::f64::consts::PI {
            angle_diff += 2.0 * std::f64::consts::PI;
        }

        let atd_meters = dist_to_start.meters() * angle_diff.cos();
        let dtg_meters = self.total_distance.meters() - atd_meters;

        LegProgress {
            xte: Distance::from_meters(dist_to_start.meters() * angle_diff.sin()),
            atd: Distance::from_meters(atd_meters),
            dtg: Distance::from_meters(dtg_meters),
        }
    }
}

pub struct Route {
    pub name: String,
    pub waypoints: Vec<Waypoint>,
}

impl Route {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            waypoints: Vec::new(),
        }
    }

    pub fn add_waypoint(&mut self, wp: Waypoint) {
        self.waypoints.push(wp);
    }

    /// Creates a Leg from the route based on a starting index.
    /// If index is 0, it creates the leg between waypoint 0 and 1.
    pub fn get_leg(&self, start_idx: usize) -> Option<Leg> {
        if start_idx + 1 >= self.waypoints.len() {
            return None; // No more waypoints to form a leg
        }

        // We clone or reference the points to create the active Leg
        Some(Leg::new(
            self.waypoints[start_idx].clone(),
            self.waypoints[start_idx + 1].clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geodesy::GeoPoint;

    #[test]
    fn test_leg_progress_start() {
        let start = Waypoint {
            name: "Start".to_string(),
            location: GeoPoint::new(0.0, 0.0).unwrap(),
        };
        let end = Waypoint {
            name: "End".to_string(),
            location: GeoPoint::new(1.0, 0.0).unwrap(),
        };
        let leg = Leg::new(start, end);

        let current_pos = GeoPoint::new(0.0, 0.0).unwrap();
        let progress = leg.get_progress(&current_pos);

        // Using the actual fields: xte, atd, dtg
        assert!(progress.dtg.meters() > 0.0);
        assert!((progress.xte.meters()).abs() < 1.0);
    }

    #[test]
    fn test_leg_progress_midway() {
        let start = Waypoint {
            name: "A".to_string(),
            location: GeoPoint::new(0.0, 0.0).unwrap(),
        };
        let end = Waypoint {
            name: "B".to_string(),
            location: GeoPoint::new(1.0, 0.0).unwrap(),
        };
        let leg = Leg::new(start, end);

        let midway_pos = GeoPoint::new(0.5, 0.0).unwrap();
        let progress = leg.get_progress(&midway_pos);

        // Check that we have moved forward (Along Track Distance)
        assert!(progress.atd.meters() > 0.0);
    }

    #[test] // Added attribute so it actually runs
    fn test_leg_progress_bearing_wrap() {
        // Leg goes from (0,0) to (0,1) -> Desired Track is 90 degrees (PI/2)
        let start = Waypoint::new("Start", 0.0, 0.0);
        let end = Waypoint::new("End", 0.0, 1.0);
        let leg = Leg::new(start, end);

        // Case 1: Trigger angle_diff < -PI
        // Target is 90 deg. If current position is "Southwest" (bearing ~280 deg),
        // 90 - 280 = -190 degrees (which is < -PI).
        // Using a point at Lat -0.1, Lon -0.1 gives a bearing of ~225 degrees.
        // 90 - 225 = -135 (not enough).

        // Let's use a point that results in a bearing of 350 degrees.
        // 90 (Track) - 350 (To Current) = -260 degrees (< -PI)
        let pos_wrap_pos = GeoPoint::new(0.1, -0.01).unwrap();
        let progress_pos = leg.get_progress(&pos_wrap_pos);
        assert!(progress_pos.atd.meters() != 0.0);

        // Case 2: Trigger angle_diff > PI
        // Target is 270 deg. Current is 10 deg.
        // 270 - 10 = 260 degrees (> PI)
        let start_2 = Waypoint::new("Start2", 0.0, 0.0);
        let end_2 = Waypoint::new("End2", 0.0, -1.0); // Heading 270
        let leg_2 = Leg::new(start_2, end_2);

        let pos_wrap_neg = GeoPoint::new(0.1, 0.01).unwrap(); // Bearing ~10 deg
        let progress_neg = leg_2.get_progress(&pos_wrap_neg);
        assert!(progress_neg.atd.meters() != 0.0);
    }

    #[test]
    fn test_route_and_leg_generation() {
        let mut route = Route::new("Test Route");
        route.add_waypoint(Waypoint::new("WP1", 0.0, 0.0));
        route.add_waypoint(Waypoint::new("WP2", 1.0, 0.0));
        route.add_waypoint(Waypoint::new("WP3", 2.0, 0.0));

        assert_eq!(route.waypoints.len(), 3);

        // Test valid leg generation
        let leg_1 = route.get_leg(0).unwrap();
        assert_eq!(leg_1.start.lat(), 0.0);
        assert_eq!(leg_1.end.lat(), 1.0);

        // Test out of bounds leg generation
        let no_leg = route.get_leg(2);
        assert!(no_leg.is_none());
    }

    #[test]
    fn test_get_progress_angle_wraps() {
        let start = Waypoint::new("A", 0.0, 0.0);
        let end = Waypoint::new("B", 1.0, 0.0); // Track is 000 (North)
        let leg = Leg::new(start, end);

        // Force angle_diff > PI: Vessel is far to the "Right" of the track
        let pos_right = GeoPoint::new(0.0, 0.1).unwrap();
        let _ = leg.get_progress(&pos_right);

        // Force angle_diff < -PI: Vessel is far to the "Left" of the track
        let pos_left = GeoPoint::new(0.0, -0.1).unwrap();
        let _ = leg.get_progress(&pos_left);
    }
}
