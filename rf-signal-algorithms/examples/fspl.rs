use rf_signal_algorithms::{free_space_path_loss_db, Distance, Frequency};

const DISTANCE_METERS: f32 = 1000.0;
const FREQ_MHZ: f32 = 1500.0;

fn main() {
    println!(
        "Free Space Loss : {}",
        free_space_path_loss_db(Frequency::with_mhz(FREQ_MHZ), Distance::with_meters(DISTANCE_METERS))
    );
}