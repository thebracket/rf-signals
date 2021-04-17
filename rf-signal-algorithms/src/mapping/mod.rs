pub mod latlon;
pub mod srtm;
pub use latlon::LatLon;
pub mod bheat;
use bheat::heat_altitude;
use rayon::prelude::*;

use crate::{
    geometry::{haversine_distance, haversine_intermediate},
    Distance,
};

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

fn highest_altitude(point: &LatLon, heat_path: &str) -> f64 {
    let altitudes = heat_altitude(point.lat(), point.lon(), heat_path)
        .unwrap_or((Distance::with_meters(0.0), Distance::with_meters(0.0)));
    f64::max(
        altitudes.0.as_meters(),
        altitudes.1.as_meters(),
    )
}

pub fn height_tile_elevations(points: &[(u32, u32, LatLon)], heat_path: &str) -> Vec<f64> {
    points
        .par_iter()
        .map(|(_, _, point)| highest_altitude(point, heat_path))
        .collect()
}

pub fn lat_lon_path_10m(src: &LatLon, dst: &LatLon) -> Vec<LatLon> {
    let d = haversine_distance(src, dst);
    let extent_step = 1.0 / (d.as_meters() / 10.0);
    let mut extent = 0.0;
    let mut path = Vec::with_capacity((d.as_meters() / 10.0) as usize);
    while extent <= 1.0 {
        let step_point = haversine_intermediate(src, dst, extent);
        path.push(step_point);
        extent += extent_step;
    }
    path
}

pub fn lat_lon_path_1m(src: &LatLon, dst: &LatLon) -> Vec<LatLon> {
    let d = haversine_distance(src, dst);
    let extent_step = 1.0 / d.as_meters();
    let mut extent = 0.0;
    let mut path = Vec::with_capacity((d.as_meters() / 10.0) as usize);
    while extent <= 1.0 {
        let step_point = haversine_intermediate(src, dst, extent);
        path.push(step_point);
        extent += extent_step;
    }
    path
}

pub fn lat_lon_vec_to_heights(points: &[LatLon], heat_path: &str) -> Vec<f64> {
    points
        .par_iter()
        .map(|point| highest_altitude(point, heat_path))
        .collect()
}

pub fn has_line_of_sight(
    los_path: &[f64],
    start_elevation: Distance,
    end_elevation: Distance,
) -> bool {
    let start_height = los_path[0] + start_elevation.as_meters();
    let end_height = end_elevation.as_meters() as u16; // Not using terrain because of confusion with clutter on lidar
    let height_step = (end_height as f64 - start_height as f64) / los_path.len() as f64;
    let mut current_height = start_height as f64;
    let mut visible = true;
    for p in los_path.iter() {
        if current_height < *p {
            visible = false;
            break;
        }
        current_height += height_step;
    }
    visible
}
