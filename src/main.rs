use rf_signal_algorithms::*;

fn main() {
    println!("Workspace root.");

    const FREQ_MHZ: f32 = 5800.0;
    const DISTANCE_METERS: f32 = 1000.0;
    const TERRAIN_STEP: f64 = DISTANCE_METERS as f64 / 200.0;
    const XMIT_HEIGHT: f32 = 30.0;
    const RECV_HEIGHT: f32 = 5.0;

    println!(
        "Cost Urban      : {}",
        cost_path_loss(FREQ_MHZ, XMIT_HEIGHT, RECV_HEIGHT, DISTANCE_METERS, 1)
    );
    println!(
        "Cost Suburban   : {}",
        cost_path_loss(FREQ_MHZ, XMIT_HEIGHT, RECV_HEIGHT, DISTANCE_METERS, 2)
    );
    println!(
        "Cost Open       : {}",
        cost_path_loss(FREQ_MHZ, XMIT_HEIGHT, RECV_HEIGHT, DISTANCE_METERS, 3)
    );
    println!(
        "ECC33 Mode 1    : {}",
        ecc33_path_loss(FREQ_MHZ, XMIT_HEIGHT, RECV_HEIGHT, DISTANCE_METERS, 1)
    );
    println!(
        "EGLI            : {}",
        egli_path_loss(FREQ_MHZ, XMIT_HEIGHT, RECV_HEIGHT, DISTANCE_METERS)
    );
    println!(
        "HATA Mode 1     : {}",
        hata_path_loss(FREQ_MHZ, XMIT_HEIGHT, RECV_HEIGHT, DISTANCE_METERS, 1)
    );
    println!(
        "HATA Mode 2     : {}",
        hata_path_loss(FREQ_MHZ, XMIT_HEIGHT, RECV_HEIGHT, DISTANCE_METERS, 2)
    );
    println!(
        "HATA Mode 3     : {}",
        hata_path_loss(FREQ_MHZ, XMIT_HEIGHT, RECV_HEIGHT, DISTANCE_METERS, 3)
    );
    println!(
        "Plane Earth     : {}",
        plane_earth_path_loss(Distance::with_meters(XMIT_HEIGHT), Distance::with_meters(RECV_HEIGHT), Distance::with_meters(DISTANCE_METERS))
    );
    for t in 1..=15 {
        println!(
            "Soil Mode {}   : {}",
            t,
            soil_path_loss(FREQ_MHZ, DISTANCE_METERS, t as f32)
        );
    }
    println!(
        "SUI Mode 1      : {}",
        sui_path_loss(FREQ_MHZ as f64, XMIT_HEIGHT as f64, RECV_HEIGHT as f64, DISTANCE_METERS as f64, 1)
    );
    println!(
        "SUI Mode 2      : {}",
        sui_path_loss(FREQ_MHZ as f64, XMIT_HEIGHT as f64, RECV_HEIGHT as f64, DISTANCE_METERS as f64, 2)
    );
    println!(
        "SUI Mode 3      : {}",
        sui_path_loss(FREQ_MHZ as f64, XMIT_HEIGHT as f64, RECV_HEIGHT as f64, DISTANCE_METERS as f64, 3)
    );
    println!(
        "Free Space Loss : {}",
        free_space_path_loss_db(FREQ_MHZ as f64, DISTANCE_METERS as f64)
    );

    let mut terrain_path = PTPPath::new(
        vec![1.0; 200],
        XMIT_HEIGHT as f64,
        RECV_HEIGHT as f64,
        TERRAIN_STEP,
    )
    .unwrap();

    let itwom_test = ItwomPointToPoint(
        &mut terrain_path,
        PTPClimate::default(),
        FREQ_MHZ as f64,
        0.5,
        0.5,
        1,
    );

    println!("ITWOM3 Loss     : {}", itwom_test.dbloss);
}
