use crate::WISP;
use rf_signal_algorithms::{Distance, Frequency, LatLon, PTPClimate, PTPPath, free_space_path_loss_db, geometry::haversine_distance, has_line_of_sight, itwom_point_to_point, lat_lon_path_10m, lat_lon_vec_to_heights, lidar::lidar_elevation, srtm::get_altitude};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ClickSite {
    pub base_height_m: f64,
    pub lidar_height_m : f64,
    pub towers: Vec<TowerEvaluation>
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TowerEvaluation {
    pub i: usize,
    pub lat: f64,
    pub lon: f64,
    pub rssi: f64,
    pub distance_km: f64,
    pub mode: String
}

pub fn evaluate_tower_click(
    pos: &LatLon,
    frequency: Frequency,
    cpe_height: f64,
    srtm_path: &str,
) -> ClickSite {
    let reader = WISP.read();
    let towers = reader
        .towers
        .iter()
        .enumerate()
        .filter(|(_, t)| {
            haversine_distance(pos, &LatLon::new(t.lat, t.lon)).as_km() < t.max_range_km
        })
        .map(|(i, t)| {
            let base_tower_height = get_altitude(&LatLon::new(t.lat, t.lon), srtm_path)
                .unwrap_or(Distance::with_meters(0.0))
                .as_meters();
            let path = lat_lon_path_10m(pos, &LatLon::new(t.lat, t.lon));
            let los_path = lat_lon_vec_to_heights(&path, srtm_path);
            let los = has_line_of_sight(
                &los_path,
                Distance::with_meters(cpe_height),
                Distance::with_meters(t.height_meters + base_tower_height),
            );
            let d = haversine_distance(pos, &LatLon::new(t.lat, t.lon));
            let (dbloss, mode) = if los || d.as_meters() < 1000.0 {
                (free_space_path_loss_db(frequency, d), "LOS Direct".to_string())
            } else {
                let mut path_as_distances: Vec<f64> = los_path.iter().map(|d| *d as f64).collect();
                let path_len = path_as_distances.len();
                path_as_distances[path_len - 1] = base_tower_height;
                let mut terrain_path = PTPPath::new(
                    path_as_distances,
                    Distance::with_meters(t.height_meters),
                    Distance::with_meters(cpe_height),
                    Distance::with_meters(10.0),
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

                (lr.dbloss, lr.mode)
            };

            let temporary_link_budget = 49.0 + 20.0 - dbloss;

            TowerEvaluation {
                i,
                lat: t.lat,
                lon: t.lon,
                rssi: temporary_link_budget,
                distance_km: d.as_km(),
                mode
            }
        })
        .collect();

    ClickSite{
        base_height_m : get_altitude(pos, srtm_path).unwrap().as_meters(),
        lidar_height_m: lidar_elevation(pos),
        towers
    }
}
