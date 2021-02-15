use rf_signal_algorithms::{hata_path_loss, Frequency, Distance, EstimateMode};

const DISTANCE_METERS: f32 = 1000.0;
const XMIT_HEIGHT: f32 = 30.0;
const RECV_HEIGHT: f32 = 5.0;
const FREQ_MHZ: f32 = 1500.0;

fn main() {
    println!(
        "HATA Mode 1     : {}",
        hata_path_loss(
            Frequency::with_mhz(FREQ_MHZ),
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS),
            EstimateMode::Urban
        )
        .unwrap()
    );
    println!(
        "HATA Mode 2     : {}",
        hata_path_loss(
            Frequency::with_mhz(FREQ_MHZ),
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS),
            EstimateMode::Suburban
        )
        .unwrap()
    );
    println!(
        "HATA Mode 3     : {}",
        hata_path_loss(
            Frequency::with_mhz(FREQ_MHZ),
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS),
            EstimateMode::Rural
        )
        .unwrap()
    );
}