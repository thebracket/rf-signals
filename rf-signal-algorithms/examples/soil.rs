use rf_signal_algorithms::{soil_path_loss, Distance, Frequency};

const DISTANCE_METERS: f32 = 1000.0;
const FREQ_MHZ: f32 = 1500.0;

fn main() {
    for t in 1..=15 {
        println!(
            "Soil Mode {}   : {}",
            t,
            soil_path_loss(
                Frequency::with_mhz(FREQ_MHZ),
                Distance::with_meters(DISTANCE_METERS),
                t as f64
            )
            .unwrap()
        );
    }
}
