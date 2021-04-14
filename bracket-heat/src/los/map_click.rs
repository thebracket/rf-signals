use crate::WISP;
use rf_signal_algorithms::{
    bheat::heat_altitude, geometry::haversine_distance, itwom_point_to_point, lat_lon_path_1m,
    lat_lon_vec_to_heights, Distance, Frequency, LatLon, PTPClimate, PTPPath,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ClickSite {
    pub base_height_m: f64,
    pub lidar_height_m: f64,
    pub towers: Vec<TowerEvaluation>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TowerEvaluation {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub rssi: f64,
    pub distance_km: f64,
    pub mode: String,
}

pub fn evaluate_tower_click(
    pos: &LatLon,
    frequency: Frequency,
    cpe_height: f64,
    heat_path: &str,
    link_budget: f64,
) -> ClickSite {
    let reader = WISP.read();
    let towers = reader
        .towers
        .iter()
        .enumerate()
        .filter(|(_, t)| {
            haversine_distance(pos, &LatLon::new(t.lat, t.lon)).as_km() < t.max_range_km
        })
        .map(|(_i, t)| {
            let base_tower_height = heat_altitude(t.lat, t.lon, heat_path)
                .unwrap_or((Distance::with_meters(0.0), Distance::with_meters(0.0)))
                .0
                .as_meters();
            let path = lat_lon_path_1m(&LatLon::new(t.lat, t.lon), pos);
            let los_path = lat_lon_vec_to_heights(&path, heat_path);
            let d = haversine_distance(pos, &LatLon::new(t.lat, t.lon));
            let (dbloss, mode) = {
                let mut path_as_distances: Vec<f64> = los_path.iter().map(|d| *d as f64).collect();
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

                (lr.dbloss, lr.mode)
            };

            let temporary_link_budget = link_budget - dbloss;

            TowerEvaluation {
                name: t.name.clone(),
                lat: t.lat,
                lon: t.lon,
                rssi: temporary_link_budget,
                distance_km: d.as_km(),
                mode,
            }
        })
        .collect();

    let h = heat_altitude(pos.lat(), pos.lon(), heat_path)
        .unwrap_or((Distance::with_meters(0), Distance::with_meters(0)));
    ClickSite {
        base_height_m: h.0.as_meters(),
        lidar_height_m: h.1.as_meters(),
        towers,
    }
}
