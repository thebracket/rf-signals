pub mod latlon;
pub mod srtm;
pub use latlon::LatLon;
use lidar::lidar_elevation;
use rayon::prelude::*;
use srtm::get_altitude;

use crate::Distance;
pub mod lidar;

/// Create a grid of LatLon entries for a bounded tile, returning (x, y, LatLon).
/// Used as a starting point for creating map tiles.
pub fn lat_lon_tile(
    swlat: f64,
    swlon: f64,
    nelat: f64,
    nelon: f64,
    tile_size: usize,
) -> Vec<(u32, u32, LatLon)> {
    let mut points = Vec::with_capacity(tile_size * tile_size);
    let mut lat = swlat;
    let lat_step = (nelat - swlat) / tile_size as f64;
    let lon_step = (nelon - swlon) / tile_size as f64;
    let mut y = 0;
    while lat < nelat - lat_step {
        let mut lon = swlon;
        let mut x = 0;
        while lon < nelon {
            points.push((x, y, LatLon::new(lat, lon)));
            lon += lon_step;
            x += 1;
        }
        lat += lat_step;
        y += 1;
    }
    points
}

pub fn height_tile_elevations(points: &[(u32, u32, LatLon)], srtm_path: &str) -> Vec<u16> {
    points
        .par_iter()
        .map(|(_, _, point)| {
            u16::max(
                get_altitude(point, &srtm_path)
                    .unwrap_or(Distance::with_meters(0))
                    .as_meters() as u16,
                lidar_elevation(point) as u16,
            )
        })
        .collect()
}
