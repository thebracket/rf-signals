use rf_signal_algorithms::{ecc33_path_loss, Frequency, Distance, EstimateMode};

const DISTANCE_METERS: f32 = 1000.0;
const XMIT_HEIGHT: f32 = 30.0;
const RECV_HEIGHT: f32 = 5.0;

fn main() {
    println!(
        "ECC33 Mode 1    : {}",
        ecc33_path_loss(
            Frequency::with_mhz(500.0),
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS),
            EstimateMode::Urban
        )
        .unwrap()
    );
}