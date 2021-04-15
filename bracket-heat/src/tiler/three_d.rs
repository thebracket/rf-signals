use rf_signal_algorithms::{bheat::heat_altitude, Distance};
use serde::{Deserialize, Serialize};
pub(crate) const TILE_SIZE: u32 = 256;
const HEIGHT_DIVISOR: f32 = 10.0;

#[derive(Clone, Serialize)]
pub struct TerrainBlob {
    pub width: u32,
    pub height: u32,
    pub terrain: Vec<f32>,
    pub clutter: Vec<f32>,
}

pub fn build_3d_heightmap(lat: f64, lon: f64, heat_path: &str) -> TerrainBlob {
    let mut terrain = Vec::new();
    let mut clutter = Vec::new();
    const STEP: f64 = 0.001;

    let mut height = 0;
    let mut lt = lat - 0.2;
    while lt < lat + 0.2 {
        let mut ln = lon - 0.2;
        while ln < lon + 0.2 {
            let h1 = heat_altitude(lt, ln, heat_path)
                .unwrap_or((Distance::with_meters(0), Distance::with_meters(0)));
            terrain.push(h1.0.as_meters() as f32 / HEIGHT_DIVISOR);
            clutter.push(h1.1.as_meters() as f32 / HEIGHT_DIVISOR);

            ln += STEP;
        }
        lt += STEP;
        height += 1;
    }

    TerrainBlob {
        width: (terrain.len() / height) as u32,
        height: height as u32,
        terrain,
        clutter,
    }
}
