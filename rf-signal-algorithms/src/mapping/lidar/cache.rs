use lazy_static::*;
use crate::LatLon;

use super::LidarIndex;
use parking_lot::Mutex;
use std::path::Path;

lazy_static! {
    static ref LIDAR_CACHE: Mutex<LidarIndex> = Mutex::new(LidarIndex::new());
}

pub fn index_all_lidar(directory: &str) {
    //let mut lock = LIDAR_CACHE.write();
    let path = Path::new(directory);
    if !path.is_dir() {
        panic!("You must specify a directory containing lidar data.")
    }

    let index_file = path.join("lidar.index");
    if index_file.exists() {
        panic!("Index file exists");
    } else {
        let now = std::time::Instant::now();
        let entries = std::fs::read_dir(path)
            .unwrap()
            .map(|p| p.unwrap().path().to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        let mut index_file = LidarIndex::new();
        entries.iter().for_each(|f| index_file.add_index_entry(f));
        index_file.bake_quadtree();
        println!("Generated index in {:?}", now.elapsed());
        *LIDAR_CACHE.lock() = index_file;
    }
}

pub fn lidar_elevation(pt: &LatLon) -> f64 {
    use super::LidarCheckerResult;
    let lidar_lock = LIDAR_CACHE.lock();
    let availability = lidar_lock.is_available(&pt.lat(), &pt.lon());
    match availability {
        LidarCheckerResult::Unavailable => 0.0,
        LidarCheckerResult::Ready => {
            // It's ready - can get it with the read lock
            lidar_lock.get_height_for_location(&pt.lat(), &pt.lon()) as f64
        }
        LidarCheckerResult::NotLoaded => {
            // Have to upgrade to a write lock to get it
            std::mem::drop(lidar_lock);
            let mut lidar_lock = LIDAR_CACHE.lock();
            lidar_lock.get_height_for_location_and_load(&pt.lat(), &pt.lon()) as f64
        }
    }
}