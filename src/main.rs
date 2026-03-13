mod comm;
mod gnc_core; // This links the folder we just set up

use crate::gnc_core::ShipState;

fn main() {
    let my_ship = ShipState::new();
    
    println!("Shipyard_GNC: System Initialized.");
    println!("Current Heading: {} degrees", my_ship.heading);
}