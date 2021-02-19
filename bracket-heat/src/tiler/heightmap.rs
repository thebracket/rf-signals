pub(crate) const TILE_SIZE: u32 = 256;

use std::io::Read;
use std::io::{Cursor, Seek, SeekFrom};

use rf_signal_algorithms::{
    height_tile_elevations, lat_lon_tile, lidar::lidar_elevation, srtm::get_altitude, Distance,
    LatLon,
};

pub fn heightmap_tile(swlat: f64, swlon: f64, nelat: f64, nelon: f64, srtm_path: &str) -> Vec<u8> {
    let mut image_data = vec![0u8; TILE_SIZE as usize * TILE_SIZE as usize * 4];

    let points = lat_lon_tile(swlat, swlon, nelat, nelon, TILE_SIZE as usize);
    let heights = height_tile_elevations(&points, srtm_path);

    let min_height = heights.iter().filter(|a| **a > 0).min().unwrap();
    let max_height = heights.iter().max().unwrap();
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
    w.seek(SeekFrom::Start(0));
    w.read_to_end(&mut out).unwrap();
    out
}
