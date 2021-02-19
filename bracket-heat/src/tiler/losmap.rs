use std::io::Read;
use std::io::{Cursor, Seek, SeekFrom};

use super::TILE_SIZE;
use rf_signal_algorithms::{Distance, LatLon, geometry::{haversine_distance, haversine_intermediate}, lidar::lidar_elevation, srtm::get_altitude};
use rocket::http::ext;
use crate::WISP;

pub fn losmap_tile(swlat: f64, swlon: f64, nelat: f64, nelon: f64, cpe_height: f64) -> Vec<u8> {
    let mut image_data = vec![0u8; TILE_SIZE as usize * TILE_SIZE as usize * 4];
    let wisp_reader = WISP.read();

    let mut points = Vec::with_capacity(TILE_SIZE as usize * TILE_SIZE as usize);
    let mut lat = swlat;
    let lat_step = (nelat - swlat) / TILE_SIZE as f64;
    let lon_step = (nelon - swlon) / TILE_SIZE as f64;
    let mut y = 0;
    while lat < nelat - lat_step {
        let mut lon = swlon;
        let mut x = 0;
        while lon < nelon {
            let current_point = LatLon::new(lat ,lon);
            let mut visible_towers = Vec::new();
            for (i,t) in wisp_reader.towers.iter().enumerate() {
                let tower_loc = LatLon::new(t.lat, t.lon);
                let d = haversine_distance(&current_point, &tower_loc);
                if d.as_km() < t.max_range_km {
                    visible_towers.push((i, d));
                }
            }
            if !visible_towers.is_empty() {
                points.push((x, y, current_point, visible_towers));
            }
            lon += lon_step;
            x += 1;
        }
        lat += lat_step;
        y += 1;
    }

    println!("{} points (of {}) are in range of a tower.", points.len(), TILE_SIZE*TILE_SIZE);

    points
        .iter()
        .for_each(|(x, y, loc, visible_towers)| {
            for (i, d) in visible_towers.iter() {
                let tower_loc = LatLon::new(wisp_reader.towers[*i].lat, wisp_reader.towers[*i].lon );
                let extent_step = f64::min(10.0 / d.as_meters(), 10.0);
                let mut extent = 0.0;
                let mut path = Vec::with_capacity((d.as_meters() / 10.0) as usize);
                while extent <= 1.0 {
                    let step_point = haversine_intermediate(&loc, &tower_loc, extent);
                    path.push(step_point);
                    extent += extent_step;
                }

                let los_path : Vec<f64> = path
                    .iter()
                    .map(|p| 
                        f64::max(
                            get_altitude(p, "z:/lidarserver/terrain").unwrap_or(Distance::with_meters(0)).as_meters(),
                            lidar_elevation(&p)
                        )
                    )
                    .collect();

                if !los_path.is_empty() {
                    let start_height = los_path[0] + cpe_height; // TODO: Replace the starting height with something defined
                    let end_height = los_path[los_path.len()-1] + wisp_reader.towers[*i].height_meters;
                    let height_step = (end_height - start_height) / los_path.len() as f64;
                    let mut current_height = start_height;
                    let mut visible = true;
                    for p in los_path.iter() {
                        if current_height < *p {
                            visible = false;
                            break;
                        }
                        current_height += height_step;
                    }

                    if visible {
                        let base = ((((TILE_SIZE - 1) - *y) as usize * 4 * TILE_SIZE as usize) + ((*x) as usize * 4)) as usize;
                        image_data[base] = 0;
                        image_data[base + 1] = 255;
                        image_data[base + 2] = 0;
                        image_data[base + 3] = 128;
                    }
                }
            }
        });

    let mut w = Cursor::new(Vec::new());
    {
        let mut encoder = png::Encoder::new(&mut w, TILE_SIZE as _, TILE_SIZE as _);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&image_data).unwrap();
    }
    let mut out = Vec::new();
    w.seek(SeekFrom::Start(0));
    w.read_to_end(&mut out).unwrap();
    out
}
