pub(crate) const TILE_SIZE: u32 = 64;

use std::io::Read;
use std::io::{Cursor, Seek, SeekFrom};

use crate::WISP;
use rf_signal_algorithms::{
    bheat::heat_altitude,
    geometry::{haversine_distance, haversine_intermediate},
    has_line_of_sight, lat_lon_path_10m, lat_lon_tile, lat_lon_vec_to_heights, Distance, LatLon,
};
use rocket::http::ext;

pub fn losmap_tile(
    swlat: f64,
    swlon: f64,
    nelat: f64,
    nelon: f64,
    cpe_height: f64,
    heat_path: &str,
) -> Vec<u8> {
    let mut image_data = vec![0u8; TILE_SIZE as usize * TILE_SIZE as usize * 4];
    let wisp_reader = WISP.read();

    let points = lat_lon_tile(swlat, swlon, nelat, nelon, TILE_SIZE as usize);
    points.iter().for_each(|(x, y, p)| {
        let is_visible = wisp_reader
            .towers
            .iter()
            .filter(|t| haversine_distance(p, &LatLon::new(t.lat, t.lon)).as_km() < t.max_range_km)
            .map(|t| {
                let base_tower_height = heat_altitude(t.lat, t.lon, heat_path)
                    .unwrap_or((Distance::with_meters(0.0), Distance::with_meters(0.0)))
                    .0
                    .as_meters();
                let path = lat_lon_path_10m(p, &LatLon::new(t.lat, t.lon));
                let los_path = lat_lon_vec_to_heights(&path, heat_path);
                has_line_of_sight(
                    &los_path,
                    Distance::with_meters(cpe_height),
                    Distance::with_meters(t.height_meters + base_tower_height),
                )
            })
            .filter(|has_los| *has_los)
            .count()
            > 0;

        if is_visible {
            let base = ((((TILE_SIZE - 1) - *y) as usize * 4 * TILE_SIZE as usize)
                + ((*x) as usize * 4)) as usize;
            image_data[base] = 0;
            image_data[base + 1] = 255;
            image_data[base + 2] = 0;
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
