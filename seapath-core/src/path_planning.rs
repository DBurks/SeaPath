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
