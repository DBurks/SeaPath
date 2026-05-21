#pragma once
#include <cstdint>

namespace seapath::executive {

// Packed attribute forces identical binary layout across Rust (laptop) and C++ (BBB)
struct __attribute__((packed)) TelemetryFrame {
    uint64_t timestamp_ns;     // Monotonic clock time-of-validity
    uint32_t sequence_number;   // Packet index to identify dropped/stale inputs
    
    // Position Data (Fed into your geodesy/calculations engines)
    double latitude_deg;       
    double longitude_deg;      
    float  current_depth_m;    
    
    // Orientation & Dynamics (Fossen State Space)
    float heading_rad;         // Yaw angle (ψ)
    float speed_knots;         // Forward velocity (u)
    
    // Guidance Demands (Outputs calculated by the C++ Node)
    float target_depth_m;      
    float target_heading_rad;  
    
    // Surface Actuation Feedback
    float stern_plane_deg;     // Vertical control surface
    float rudder_deg;          // Horizontal control surface
};

} // namespace seapath::executive