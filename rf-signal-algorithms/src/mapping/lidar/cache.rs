use lazy_static::*;
use crate::LatLon;

use super::{LidarIndex, LidarHeader, LidarFile};
use parking_lot::RwLock;
use std::path::Path;
use rayon::prelude::*;
use memmap::Mmap;

lazy_static! {
    static ref LIDAR_CACHE: RwLock<LidarIndex> = RwLock::new(LidarIndex::new());
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
        println!("No cached index file found - rebuilding");
        let now = std::time::Instant::now();
        let entries = std::fs::read_dir(path)
            .unwrap()
            .map(|p| p.unwrap().path().to_str().unwrap().to_string())
            .collect::<Vec<String>>();
        println!("Found {} LIDAR files to index. Search took {:?}", entries.len(), now.elapsed());

        let mut headers : Vec<(LidarHeader, Mmap)> = entries
            .par_iter()
            .map(|filename| {
                let (hdr, memory) = LidarFile::header_and_mmap(Path::new(filename));
                (
                    hdr,
                    memory
                )
            })
            .collect();
        println!("Mapped headers, {:?}", now.elapsed());

        let mut index_file = LidarIndex::new();
        while !headers.is_empty() {
            let (header, memory) = headers.pop().unwrap();
            index_file.add_index_entry(header, memory);
        }
        println!("Parsed headers, {:?}", now.elapsed());
            index_file.bake_quadtree();
        println!("Generated index in {:?}", now.elapsed());
        *LIDAR_CACHE.write() = index_file;
    }
}

pub fn lidar_elevation(pt: &LatLon) -> f64 {
    let lidar_lock = LIDAR_CACHE.read();
    let availability = lidar_lock.is_available(&pt.lat(), &pt.lon());
    match availability {
        false => 0.0,
        true => {
            // It's ready - can get it with the read lock
            lidar_lock.get_height_for_location(&pt.lat(), &pt.lon()) as f64
        }
    }
}