use rf_signal_algorithms::{egli_path_loss, Frequency, Distance};

const DISTANCE_METERS: f32 = 1000.0;
const XMIT_HEIGHT: f32 = 30.0;
const RECV_HEIGHT: f32 = 5.0;
const FREQ_MHZ: f32 = 1500.0;

fn main() {
    println!(
        "EGLI            : {}",
        egli_path_loss(
            Frequency::with_mhz(FREQ_MHZ),
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS),
        )
        .unwrap()
    );
}