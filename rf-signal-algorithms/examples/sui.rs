use rf_signal_algorithms::{sui_path_loss, Distance, EstimateMode, Frequency};

const DISTANCE_METERS: f32 = 1000.0;
const FREQ_MHZ: f32 = 1500.0;
const XMIT_HEIGHT: f32 = 30.0;
const RECV_HEIGHT: f32 = 5.0;

fn main() {
    println!(
        "SUI Mode 1      : {}",
        sui_path_loss(
            Frequency::with_mhz(FREQ_MHZ + 2000.0),
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS),
            EstimateMode::Urban
        )
        .unwrap()
    );
    println!(
        "SUI Mode 2      : {}",
        sui_path_loss(
            Frequency::with_mhz(FREQ_MHZ + 2000.0),
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS),
            EstimateMode::Suburban
        )
        .unwrap()
    );
    println!(
        "SUI Mode 3      : {}",
        sui_path_loss(
            Frequency::with_mhz(FREQ_MHZ + 2000.0),
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS),
            EstimateMode::Rural
        )
        .unwrap()
    );
}
