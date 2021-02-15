use rf_signal_algorithms::{PTPClimate, PTPPath, itwom_point_to_point};

const DISTANCE_METERS: f32 = 1000.0;
const FREQ_MHZ: f32 = 1500.0;
const XMIT_HEIGHT: f32 = 30.0;
const RECV_HEIGHT: f32 = 5.0;
const TERRAIN_STEP: f64 = DISTANCE_METERS as f64 / 200.0;

fn main() {
    let mut terrain_path = PTPPath::new(
        vec![1.0; 200],
        XMIT_HEIGHT as f64,
        RECV_HEIGHT as f64,
        TERRAIN_STEP,
    )
    .unwrap();

    let itwom_test = itwom_point_to_point(
        &mut terrain_path,
        PTPClimate::default(),
        FREQ_MHZ as f64,
        0.5,
        0.5,
        1,
    );

    println!("ITWOM3 Loss     : {}", itwom_test.dbloss);
}