use rf_signal_algorithms::{cost_path_loss, Frequency, Distance, EstimateMode};

const DISTANCE_METERS: f32 = 1000.0;
const XMIT_HEIGHT: f32 = 30.0;
const RECV_HEIGHT: f32 = 5.0;

fn main() {
    println!(
        "Cost Urban      : {}",
        cost_path_loss(
            Frequency::with_mhz(1700.0),
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS),
            EstimateMode::Urban
        )
        .unwrap()
    );
    println!(
        "Cost Suburban   : {}",
        cost_path_loss(
            Frequency::with_mhz(1700.0),
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS),
            EstimateMode::Suburban
        )
        .unwrap()
    );
    println!(
        "Cost Open       : {}",
        cost_path_loss(
            Frequency::with_mhz(1700.0),
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS),
            EstimateMode::Rural
        )
        .unwrap()
    );
}