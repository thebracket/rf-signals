use lru::LruCache;
use super::SrtmTile;
use crate::LatLon;
use lazy_static::*;
use memmap::{ Mmap, MmapOptions };
use parking_lot::Mutex;
use std::fs::File;
use crate::Distance;

lazy_static! {
    static ref TILE_CACHE : Mutex<LruCache<SrtmTile, Mmap>> = Mutex::new(LruCache::new(20));
}

pub fn get_altitude(loc: &LatLon, terrain_path: &str) -> Option<Distance> {
    // Check for already cached tiles
    let in_cache = check_existing(loc);
    if in_cache.is_some() {
        return in_cache;
    }

    // If we've got this far, it isn't cached. Try and load it.
    if let Some(tile) = SrtmTile::check_availability(loc, terrain_path) {
        let mut cache_writer = TILE_CACHE.lock();
        if let Ok(cache_file) = File::open(&tile.filename(terrain_path)) {
            let mapped_file = unsafe { MmapOptions::new().map(&cache_file).unwrap() };
            let elevation = get_elevation(loc, &tile, &mapped_file);
            cache_writer.put(tile, mapped_file);
            return Some(elevation);
        } else {
            return None;
        }
    }

    // Failure
    None
}

fn check_existing(loc: &LatLon) -> Option<Distance> {
    let mut cache_reader = TILE_CACHE.lock();

    let third = loc.to_srtm_third();
    if let Some(mm) = cache_reader.get(&third) {
        return Some(get_elevation(loc, &third, &mm));
    }

    let three = loc.to_srtm3();
    if let Some(mm) = cache_reader.get(&three) {
        return Some(get_elevation(loc, &three, &mm));
    }

    let one = loc.to_srtm1();
    if let Some(mm) = cache_reader.get(&one) {
        return Some(get_elevation(loc, &one, &mm));
    }

    None
}

fn get_elevation(loc: &LatLon, tile: &SrtmTile, memory: &Mmap) -> Distance {
    let floor = loc.floor();
    let offset = match tile {
        SrtmTile::Srtm1 { .. } => {
            const BYTES_PER_SAMPLE: usize = 2;
            const N_SAMPLES: usize = 1201;
            const SAMPLES_PER_DEGREE: usize = N_SAMPLES - 1;
            let row = ((floor.lat() + 1.0 - loc.lat()) * SAMPLES_PER_DEGREE as f64).round()
                as usize;
            let col =
                ((loc.lon() - floor.lon()) * SAMPLES_PER_DEGREE as f64).round() as usize;
            BYTES_PER_SAMPLE * ((row * N_SAMPLES) + col)
        }
        SrtmTile::Srtm3 { .. } => {
            const BYTES_PER_SAMPLE: usize = 2;
            const N_SAMPLES: usize = 3601;
            const SAMPLES_PER_DEGREE: usize = N_SAMPLES - 1;
            let row = ((floor.lat() + 1.0 - loc.lat()) * SAMPLES_PER_DEGREE as f64).round()
                as usize;
            let col =
                ((loc.lon() - floor.lon()) * SAMPLES_PER_DEGREE as f64).round() as usize;
            BYTES_PER_SAMPLE * ((row * N_SAMPLES) + col)
        }
        SrtmTile::SrtmThird {
            lat_tile, lon_tile, ..
        } => {
            const SAMPLES_PER_DEGREE: usize = 1200;
            const SAMPLES_PER_EXTENT: usize = 1201;
            const BYTES_PER_SAMPLE: usize = 2;
            let lat_extent_base_10 = loc.lat() * 10.0 - floor.lat() * 10.0;
            let lat_extent_base_9 = ((lat_extent_base_10 / 10.0) * 9.0) + 1.0;
            let lon_extent_base_10 = loc.lon() * 10.0 - floor.lon() * 10.0;
            let lon_extent_base_9 = ((lon_extent_base_10 / 10.0) * 9.0) + 1.0;
            let lat_percent = lat_extent_base_9 - *lat_tile as f64;
            let lon_percent = lon_extent_base_9 - *lon_tile as f64;
            let row = ((1.0 - lat_percent) * SAMPLES_PER_DEGREE as f64).round() as usize;
            let col = (lon_percent * SAMPLES_PER_DEGREE as f64).round() as usize;
            BYTES_PER_SAMPLE * ((row * SAMPLES_PER_EXTENT) + col)
        }
    };

    let h = {
        let high_byte = *memory.get(offset + 1).unwrap();
        let low_byte = *memory.get(offset).unwrap();
        ((low_byte as u16) << 8) | high_byte as u16
    };
    Distance::with_meters(h)
}