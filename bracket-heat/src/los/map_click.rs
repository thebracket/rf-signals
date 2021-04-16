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
    pub tower: String,
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
    let services = crate::calculators::services_in_range(pos);
    let evaluation = crate::calculators::evaluate_wireless_services(pos, &services, heat_path);

    let towers = evaluation
        .iter()
        .map(|e| TowerEvaluation {
            tower: e.tower.clone(),
            name: format!("{}:{} @{}m", e.tower, e.name, e.cpe_height),
            lat: e.tower_pos.lat(),
            lon: e.tower_pos.lon(),
            rssi: e.signal,
            distance_km: e.range_km,
            mode: e.mode.clone(),
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
