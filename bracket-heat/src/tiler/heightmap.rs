pub(crate) const TILE_SIZE: u32 = 256;
pub(crate) const DETAIL_SIZE: u32 = 256;

use std::io::Read;
use std::io::{Cursor, Seek, SeekFrom};

use rf_signal_algorithms::{height_tile_elevations, lat_lon_tile};

pub fn heightmap_tile(swlat: f64, swlon: f64, nelat: f64, nelon: f64, heat_path: &str) -> Vec<u8> {
    let mut image_data = vec![0u8; TILE_SIZE as usize * TILE_SIZE as usize * 4];

    let points = lat_lon_tile(swlat, swlon, nelat, nelon, TILE_SIZE as usize);
    let heights = height_tile_elevations(&points, heat_path);

    let min_height = heights.iter().filter(|a| **a > 0).min().unwrap_or(&0);
    let max_height = heights.iter().max().unwrap_or(&1);
    let h_scale = 255.0 / (*max_height as f64 - *min_height as f64);

    heights.iter().enumerate().for_each(|(i, h)| {
        if *h > 0 {
            let x = points[i].0;
            let y = points[i].1;
            let base = ((((TILE_SIZE - 1) - y) as usize * 4 * TILE_SIZE as usize)
                + (x as usize * 4)) as usize;
            let n = ((*h as f64 - *min_height as f64) * h_scale) as u8;
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
    w.seek(SeekFrom::Start(0)).unwrap();
    w.read_to_end(&mut out).unwrap();
    out
}

pub fn heightmap_detail(swlat: f64, swlon: f64, nelat: f64, nelon: f64, heat_path: &str) -> Vec<u8> {
    let mut image_data = vec![0u8; DETAIL_SIZE as usize * DETAIL_SIZE as usize * 4];

    let points = lat_lon_tile(swlat, swlon, nelat, nelon, DETAIL_SIZE as usize);
    let heights = height_tile_elevations(&points, heat_path);

    let min_height = heights.iter().filter(|a| **a > 0).min().unwrap_or(&0);
    let max_height = heights.iter().max().unwrap_or(&1);
    let h_scale = 255.0 / (*max_height as f64 - *min_height as f64);

    heights.iter().enumerate().for_each(|(i, h)| {
        if *h > 0 {
            let x = points[i].0;
            let y = points[i].1;
            let base = ((((DETAIL_SIZE - 1) - y) as usize * 4 * DETAIL_SIZE as usize)
                + (x as usize * 4)) as usize;
            let n = ((*h as f64 - *min_height as f64) * h_scale) as u8;
            if base+3 < image_data.len() {
                image_data[base] = n;
                image_data[base + 1] = n;
                image_data[base + 2] = n;
                image_data[base + 3] = 255;
            }
        }
    });

    let mut w = Cursor::new(Vec::new());
    {
        let mut encoder = png::Encoder::new(&mut w, DETAIL_SIZE as _, DETAIL_SIZE as _);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&image_data).unwrap();
    }
    let mut out = Vec::new();
    w.seek(SeekFrom::Start(0)).unwrap();
    w.read_to_end(&mut out).unwrap();
    out
}