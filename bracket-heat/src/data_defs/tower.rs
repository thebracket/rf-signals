use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Tower {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub height_meters: f64,
    pub max_range_km: f64,
}
