use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Tower {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub height_meters: f64,
    pub max_range_km: f64,
    pub access_points: Vec<AP>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AP {
    pub name: String,
    pub frequency_ghz: f64,
    pub max_range_km: f64,
    pub link_budget: f64,
}
