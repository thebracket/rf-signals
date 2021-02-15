use rf_signal_algorithms::{itwom_point_to_point, Distance, Frequency, PTPClimate, PTPPath};

const FREQ_MHZ: f32 = 5840.0;
const XMIT_HEIGHT: f32 = 3.0;
const RECV_HEIGHT: f32 = 30.0;
const TERRAIN_STEP: f64 = 10.0;

fn main() {
    let mut terrain_path = PTPPath::new(
        vec![1.0; 200],
        Distance::with_meters(XMIT_HEIGHT),
        Distance::with_meters(RECV_HEIGHT),
        Distance::with_meters(TERRAIN_STEP),
    )
    .unwrap();

    let itwom_test = itwom_point_to_point(
        &mut terrain_path,
        PTPClimate::default(),
        Frequency::with_mhz(FREQ_MHZ),
        0.5,
        0.5,
        1,
    );

    println!("ITWOM3 Loss     : {}", itwom_test.dbloss);
    println!("ITWOM3 Mode     : {}", itwom_test.mode);
    println!("ITWOM3 Error #  : {}", itwom_test.error_num);
    // Ideally 0.
}
