use geo_types::Point;
use las::{point::Classification, Header, Read, Reader};
use proj::*;
mod tile_writer;
use rf_signal_algorithms::srtm::get_altitude;
use rf_signal_algorithms::*;
use std::{collections::HashMap, fs::read_dir, path::Path};
use tile_writer::*;

const LIDAR_PATH: &str = "/home/herbert/lidar/";

fn get_projection_string(header: &Header) -> Option<String> {
    let mut result = None;
    for v in header.all_vlrs() {
        if v.description == "GeoTiff ASCII parameters" {
            let mut s = String::new();
            v.data.iter().for_each(|c| {
                s.push(*c as char);
            });

            let mut split = s.split("|");

            result = Some(split.nth(0).unwrap().to_string());
        }
    }
    result
}

fn store_altitude(
    t: &mut MapTile,
    lat: f64,
    lon: f64,
    classification: &Classification,
    altitude_m: u16,
) {
    let index = t.index(lat, lon);
    match classification {
        Classification::Ground => t.store_ground(index, altitude_m),
        _ => t.store_clutter(index, altitude_m),
    }
}

fn main() {
    let dir = Path::new(LIDAR_PATH);
    if !dir.is_dir() {
        panic!("Must be a directory");
    }

    for entry in read_dir(&dir).unwrap() {
        if let Ok(entry) = entry {
            if entry.path().is_file() && entry.path().extension().unwrap() == "las" {
                println!("Working on {}...", entry.path().to_str().as_ref().unwrap());
                let mut reader = Reader::from_path(entry.path().to_str().as_ref().unwrap())
                    .expect("Unable to open LAS file");
                let projection = get_projection_string(reader.header()).unwrap();
                let to = "WGS84";
                let converter = Proj::new_known_crs(&projection, &to, None).unwrap();

                let mut tile_cache = HashMap::<String, MapTile>::new();

                /*println!("{}..{}", reader.header().bounds().min.z, reader.header().bounds().max.z);
                let altitude_conversion = if reader.header().bounds().max.z > 500.0 {
                    println!("Looks like feet");
                    0.3048
                } else {
                    println!("Looks like meters");
                    1.0
                };*/

                let mut altitude_conversion = 0.3048;
                reader
                    .points()
                    .filter(|p| p.is_ok())
                    .map(|p| p.unwrap())
                    .filter(|p| p.classification == Classification::Ground)
                    .take(3)
                    .for_each(|p| {
                        let tmp = converter.convert(Point::new(p.x, p.y)).unwrap();
                        let ll = LatLon::new(tmp.lat(), tmp.lng());
                        let h = get_altitude(
                            &ll,
                            "/home/herbert/lidarserver/terrain"
                        ).unwrap_or(Distance::with_meters(0)).as_meters();
                        let margin = h / 10.0;
                        if p.z >= h-margin && p.z <= h+margin {
                            altitude_conversion = 1.0;
                            println!("It appears to be in feet. Known height: {}m, found: {}m. Margin: {}", h, p.z * altitude_conversion as f64, margin);
                        } else {
                            altitude_conversion = 0.3048;
                            println!("It appears to be in feet. Known height: {}m, found: {}m", h, p.z * altitude_conversion as f64);
                        }
                    });

                reader
                    .points()
                    .filter(|p| p.is_ok())
                    .map(|p| p.unwrap())
                    .for_each(|p| {
                        let tmp = converter.convert(Point::new(p.x, p.y)).unwrap();
                        let filename = MapTile::get_tile_name(tmp.lat(), tmp.lng());
                        if let Some(t) = tile_cache.get_mut(&filename) {
                            store_altitude(
                                t,
                                tmp.lat(),
                                tmp.lng(),
                                &p.classification,
                                (p.z as f32 * 10.0 * altitude_conversion) as u16,
                            );
                        } else {
                            let mut tile = MapTile::get_tile(tmp.lat(), tmp.lng());
                            store_altitude(
                                &mut tile,
                                tmp.lat(),
                                tmp.lng(),
                                &p.classification,
                                (p.z as f32 * 10.0 * altitude_conversion) as u16,
                            );
                            tile_cache.insert(filename, tile);
                        }
                    });
                tile_cache.iter().for_each(|(_, v)| {
                    v.save();
                    v.save_png();
                });
            }
        }
    }
}
