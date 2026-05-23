mod network_types;
mod kinematics; // Declare the new module

use network_types::{TelemetryFrame, PackedWireParser};
use kinematics::VesselKinematics;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::time::interval;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Shipyard_GNC Simulator Engine Online ===");

    let local_addr: SocketAddr = "127.0.0.1:8081".parse()?;  
    let target_addr: SocketAddr = "127.0.0.1:8080".parse()?; 
    
    let socket = UdpSocket::bind(local_addr).await?;
    println!("[NET] Connected to network line. Streaming to: {}", target_addr);

    // Initialize our physical plant kinematics state
    let mut vessel = VesselKinematics::new(40.0, -105.0, 0.0);
    
    // Track current actuator positions tracking back from C++ control driver
    let mut current_rudder_demand_deg = 0.0f32;
    let mut current_plane_demand_deg = 0.0f32;
    let mut seq = 0u32;

    let start_time = Instant::now();
    let dt = 0.020f32; // 20 milliseconds step
    let mut frame_ticker = interval(Duration::from_millis(20)); 
    let mut receive_buffer = [0u8; 128]; 

loop {
        frame_ticker.tick().await;
        seq += 1;

        // 1. STEP PLANT KINEMATICS
        vessel.update(current_rudder_demand_deg, dt);

        // 2. PACK AND STREAM OUTBOUND TELEMETRY
        let telemetry = TelemetryFrame {
            timestamp_ns: start_time.elapsed().as_nanos() as u64,
            sequence_number: seq,
            latitude_deg: vessel.latitude_deg,
            longitude_deg: vessel.longitude_deg,
            current_depth_m: 15.0,     
            heading_rad: vessel.heading_rad,
            speed_knots: vessel.surge_knots,
            target_depth_m: 0.0,      
            target_heading_rad: 0.0,  
            stern_plane_deg: current_plane_demand_deg,     
            rudder_deg: current_rudder_demand_deg,          
        };

        let outbound_bytes = PackedWireParser::pack(&telemetry);
        let _ = socket.send_to(&outbound_bytes, target_addr).await;

        // 3. CAPTURE ACTUATOR RESPONSES (FULLY FLATTENED)
        let (bytes_read, _) = match socket.try_recv_from(&mut receive_buffer) {
            Ok(result) => result,
            Err(_) => continue, // No packet available this frame, jump to next loop tick safely
        };

        if bytes_read < 56 {
            continue;
        }

        let cmd_frame = match PackedWireParser::unpack(&receive_buffer[0..56]) {
            Some(frame) => frame,
            None => continue,
        };

        current_rudder_demand_deg = cmd_frame.rudder_deg;
        current_plane_demand_deg = cmd_frame.stern_plane_deg;

        println!(
            "[HIL FEEDBACK] Seq: {} | Lat: {:.6} | Lon: {:.6} | Hdg: {:.4} rad",
            cmd_frame.sequence_number,
            vessel.latitude_deg,
            vessel.longitude_deg,
            vessel.heading_rad
        );
    }
}