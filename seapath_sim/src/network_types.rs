#[derive(Debug, Clone, Copy)]
pub struct TelemetryFrame {
    pub timestamp_ns: u64,
    pub sequence_number: u32,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub current_depth_m: f32,
    pub heading_rad: f32,
    pub speed_knots: f32,
    pub target_depth_m: f32,
    pub target_heading_rad: f32,
    pub stern_plane_deg: f32,
    pub rudder_deg: f32,
}

pub struct PackedWireParser;

impl PackedWireParser {
    /// Extracts data sequentially from a raw byte buffer matching C++ layout limits
    pub fn unpack(bytes: &[u8]) -> Option<TelemetryFrame> {
        if bytes.len() < 56 { 
            return None; 
        }

        Some(TelemetryFrame {
            timestamp_ns:       u64::from_le_bytes(bytes[0..8].try_into().ok()?),
            sequence_number:    u32::from_le_bytes(bytes[8..12].try_into().ok()?),
            latitude_deg:       f64::from_le_bytes(bytes[12..20].try_into().ok()?),
            longitude_deg:      f64::from_le_bytes(bytes[20..28].try_into().ok()?),
            current_depth_m:    f32::from_le_bytes(bytes[28..32].try_into().ok()?),
            heading_rad:        f32::from_le_bytes(bytes[32..36].try_into().ok()?),
            speed_knots:        f32::from_le_bytes(bytes[36..40].try_into().ok()?),
            target_depth_m:     f32::from_le_bytes(bytes[40..44].try_into().ok()?),
            target_heading_rad: f32::from_le_bytes(bytes[44..48].try_into().ok()?),
            stern_plane_deg:    f32::from_le_bytes(bytes[48..52].try_into().ok()?),
            rudder_deg:         f32::from_le_bytes(bytes[52..56].try_into().ok()?),
        })
    }

    /// Packs standard aligned structures down to a explicit 56-byte frame block
    pub fn pack(frame: &TelemetryFrame) -> [u8; 56] {
        let mut buf = [0u8; 56];
        buf[0..8].copy_from_slice(&frame.timestamp_ns.to_le_bytes());
        buf[8..12].copy_from_slice(&frame.sequence_number.to_le_bytes());
        buf[12..20].copy_from_slice(&frame.latitude_deg.to_le_bytes());
        buf[20..28].copy_from_slice(&frame.longitude_deg.to_le_bytes());
        buf[28..32].copy_from_slice(&frame.current_depth_m.to_le_bytes());
        buf[32..36].copy_from_slice(&frame.heading_rad.to_le_bytes());
        buf[36..40].copy_from_slice(&frame.speed_knots.to_le_bytes());
        buf[40..44].copy_from_slice(&frame.target_depth_m.to_le_bytes());
        buf[44..48].copy_from_slice(&frame.target_heading_rad.to_le_bytes());
        buf[48..52].copy_from_slice(&frame.stern_plane_deg.to_le_bytes());
        buf[52..56].copy_from_slice(&frame.rudder_deg.to_le_bytes());
        buf
    }
}