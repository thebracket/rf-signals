use std::io::{Cursor, Seek, SeekFrom};
use std::io::Read;

use super::TILE_SIZE;
use rf_signal_algorithms::{srtm::get_altitude, Distance, LatLon};

pub fn heightmap_tile(swlat: f64, swlon: f64, nelat: f64, nelon: f64) -> Vec<u8> {
    let mut image_data = vec![0u8; TILE_SIZE as usize * TILE_SIZE as usize * 4];

    let mut points = Vec::with_capacity(TILE_SIZE as usize * TILE_SIZE as usize);
    let mut lat = swlat;
    let lat_step = (nelat - swlat) / TILE_SIZE as f64;
    let lon_step = (nelon - swlon) / TILE_SIZE as f64;
    println!("Lat: {} .. {}, step {}", nelat, swlat, lat_step);
    println!("Lon: {} .. {}, step {}", nelon, swlon, lon_step);
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

    let heights: Vec<(u32, u32, f64)> = points
        .iter()
        .map(|(x, y, p)| {
            (
                *x,
                *y,
                get_altitude(p, "z:/lidarserver/terrain")
                    .unwrap_or(Distance::with_meters(0))
                    .as_meters(),
            )
        })
        .collect();

    let min_height = heights
        .iter()
        .filter(|a| a.2 > 0.0)
        .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap()
        .2;
    let max_height = heights
        .iter()
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap()
        .2;
    let h_scale = 255.0 / (max_height - min_height);
    println!("Range: {}..{}, scale: {}", min_height, max_height, h_scale);

    heights.iter().for_each(|(x, y, h)| {
        if *h > 0.0 {
            let base = ((((TILE_SIZE-1) - *y) as usize * 4 * TILE_SIZE as usize) + ((*x) as usize * 4)) as usize;
            let n = ((*h - min_height) * h_scale) as u8;
            image_data[base] = n;
            image_data[base + 1] = n;
            image_data[base + 2] = n;
            image_data[base + 3] = 128;
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
