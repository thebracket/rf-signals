use crate::WISP;
use rf_signal_algorithms::{
    bheat::heat_altitude, free_space_path_loss_db, fresnel_radius, geometry::haversine_distance,
    has_line_of_sight, itwom_point_to_point, lat_lon_path_1m, lat_lon_vec_to_heights, Distance,
    Frequency, LatLon, PTPClimate, PTPPath,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct LineOfSightPlot {
    pub tower_base_height: f64,
    pub srtm: Vec<f64>,
    pub lidar: Vec<f64>,
    pub fresnel: Vec<f64>,
    pub dbloss: f64,
    pub mode: String,
    pub distance_m: f64,
}

pub fn los_plot(
    pos: &LatLon,
    tower_index: usize,
    cpe_height: f64,
    frequency: Frequency,
    heat_path: &str,
) -> LineOfSightPlot {
    let reader = crate::WISP.read();
    let t = &reader.towers[tower_index];
    let d = haversine_distance(pos, &LatLon::new(t.lat, t.lon));
    let base_tower_height = heat_altitude(t.lat, t.lon, heat_path)
        .unwrap_or((Distance::with_meters(0.0), Distance::with_meters(0.0)))
        .0
        .as_meters();
    let path = lat_lon_path_1m(&LatLon::new(t.lat, t.lon), pos); // Tower is 1st

    // Calculate the LoS and loss - should be cached data
    let los_path = lat_lon_vec_to_heights(&path, heat_path);
    let (dbloss, mode) = {
        let mut path_as_distances: Vec<f64> = los_path.iter().map(|d| *d as f64).collect();
        let path_len = path_as_distances.len();
        path_as_distances[0] = base_tower_height;
        let mut terrain_path = PTPPath::new(
            path_as_distances,
            Distance::with_meters(t.height_meters),
            Distance::with_meters(cpe_height),
            Distance::with_meters(1.0),
        )
        .unwrap();

        let lr = itwom_point_to_point(
            &mut terrain_path,
            PTPClimate::default(),
            frequency,
            0.5,
            0.5,
            1,
        );

        (lr.dbloss, format!("{} ({})", lr.mode, lr.error_num))
    };

    // Expand out the srtm, lidar and fresnel fields
    let mut srtm = Vec::new();
    let mut lidar = Vec::new();
    let mut fresnel = Vec::new();

    let mut walker = 0.0;
    path.iter().for_each(|loc| {
        let h = heat_altitude(loc.lat(), loc.lon(), heat_path)
            .unwrap_or((Distance::with_meters(0), Distance::with_meters(0)));
        srtm.push(h.0.as_meters());
        lidar.push(h.1.as_meters());
        fresnel.push(fresnel_radius(walker, d.as_meters() - walker, frequency.as_mhz()) * 0.6);
        walker += 1.0;
    });

    LineOfSightPlot {
        tower_base_height: base_tower_height,
        srtm,
        lidar,
        fresnel,
        dbloss,
        mode,
        distance_m: d.as_meters(),
    }
}
