use rf_signal_algorithms::{plane_earth_path_loss, Distance};

const DISTANCE_METERS: f32 = 1000.0;
const XMIT_HEIGHT: f32 = 30.0;
const RECV_HEIGHT: f32 = 5.0;

fn main() {
    println!(
        "Plane Earth     : {}",
        plane_earth_path_loss(
            Distance::with_meters(XMIT_HEIGHT),
            Distance::with_meters(RECV_HEIGHT),
            Distance::with_meters(DISTANCE_METERS)
        )
    );
}
