use rf_signal_algorithms::*;

fn main() {
    println!("Workspace root.");

    let mut elevations = vec![1.0; 200];
        elevations[100] = 110.0;
        elevations[150] = 409.0;
        let mut terrain_path = PTPPath::new(elevations, 100.0, 100.0, 10.0).unwrap();

        let itwom_test = ItwomPointToPoint(
            &mut terrain_path,
            PTPClimate::default(), 
            5800.0,
            0.5,
            0.5,
            1
        );
    println!("{:#?}", itwom_test);
}