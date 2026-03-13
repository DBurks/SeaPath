// This is the universal contract for any hardware bus (CAN, 1553, etc.)
pub trait ShipBus {
    fn send_command(&self, value: f32) -> Result<(), String>;
}

// We will add 'pub mod mavlink;' here once the AI writes that specific driver.