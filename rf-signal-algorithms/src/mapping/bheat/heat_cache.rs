use super::MapTile;
use crate::Distance;
use lazy_static::*;
use memmap::{Mmap, MmapOptions};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::fs::File;

lazy_static! {
    static ref HEAT_CACHE: RwLock<HashMap<String, Option<Mmap>>> = RwLock::new(HashMap::new());
}

pub fn heat_altitude(lat: f64, lon: f64, heat_path: &str) -> Option<(Distance, Distance)> {
    let filename = MapTile::get_tile_name(lat, lon, heat_path);

    let read_lock = HEAT_CACHE.read();
    if let Some(tile_file) = read_lock.get(&filename) {
        if let Some(mm) = tile_file {
            Some(get_elevation(lat, lon, mm))
        } else {
            None
        }
    } else {
        // We don't have it - try and load it
        std::mem::drop(read_lock);
        let mut write_lock = HEAT_CACHE.write();
        if let Ok(cache_file) = File::open(&filename) {
            let mapped_file = unsafe { MmapOptions::new().map(&cache_file).unwrap() };
            let elevation = get_elevation(lat, lon, &mapped_file);
            write_lock.insert(filename, Some(mapped_file));
            Some(elevation)
        } else {
            write_lock.insert(filename, None);
            None
        }
    }
}

fn get_elevation(lat: f64, lon: f64, memory: &Mmap) -> (Distance, Distance) {
    let index = MapTile::index(lat, lon);
    let offset = index * 2;
    let heights = bytemuck::cast_slice::<u8, u16>(&memory);
    let ground = heights[offset] / 10;
    let clutter = heights[offset + 1] / 10;
    (
        Distance::with_meters(ground),
        Distance::with_meters(clutter),
    )
}
