use rf_signal_algorithms::free_space_path_loss_db;

const DISTANCE_METERS: f32 = 1000.0;
const FREQ_MHZ: f32 = 1500.0;

fn main() {
    println!(
        "Free Space Loss : {}",
        free_space_path_loss_db(FREQ_MHZ as f64, DISTANCE_METERS as f64)
    );
}