pub(crate) const TILE_SIZE: u32 = 64;

use std::io::Read;
use std::io::{Cursor, Seek, SeekFrom};

use crate::WISP;
use rf_signal_algorithms::{
    free_space_path_loss_db,
    geometry::{haversine_distance, haversine_intermediate},
    has_line_of_sight, itwom_point_to_point, lat_lon_path_10m, lat_lon_tile,
    lat_lon_vec_to_heights,
    lidar::lidar_elevation,
    srtm::get_altitude,
    Distance, Frequency, LatLon, PTPClimate, PTPPath,
};
use rocket::http::ext;

pub fn signalmap_tile(
    swlat: f64,
    swlon: f64,
    nelat: f64,
    nelon: f64,
    cpe_height: f64,
    frequency: f64,
    srtm_path: &str,
) -> Vec<u8> {
    let mut image_data = vec![0u8; TILE_SIZE as usize * TILE_SIZE as usize * 4];
    let wisp_reader = WISP.read();

    let points = lat_lon_tile(swlat, swlon, nelat, nelon, TILE_SIZE as usize);
    points.iter().for_each(|(x, y, p)| {
        let dbloss = wisp_reader
            .towers
            .iter()
            .filter(|t| haversine_distance(p, &LatLon::new(t.lat, t.lon)).as_km() < t.max_range_km)
            .map(|t| {
                let base_tower_height = get_altitude(&LatLon::new(t.lat, t.lon), srtm_path)
                    .unwrap_or(Distance::with_meters(0.0))
                    .as_meters();
                let path = lat_lon_path_10m(p, &LatLon::new(t.lat, t.lon));
                let los_path = lat_lon_vec_to_heights(&path, srtm_path);
                let los = has_line_of_sight(
                    &los_path,
                    Distance::with_meters(cpe_height),
                    Distance::with_meters(t.height_meters + base_tower_height),
                );
                let d = haversine_distance(p, &LatLon::new(t.lat, t.lon));
                if los || d.as_meters() < 1000.0 {
                    free_space_path_loss_db(Frequency::with_ghz(frequency), d)
                } else {
                    let mut path_as_distances: Vec<f64> =
                        los_path.iter().map(|d| *d as f64).collect();
                    let path_len = path_as_distances.len();
                    path_as_distances[path_len - 1] = base_tower_height;
                    let mut terrain_path = PTPPath::new(
                        path_as_distances,
                        Distance::with_meters(t.height_meters),
                        Distance::with_meters(cpe_height),
                        Distance::with_meters(10.0),
                    )
                    .unwrap();

                    let lr = itwom_point_to_point(
                        &mut terrain_path,
                        PTPClimate::default(),
                        Frequency::with_ghz(frequency),
                        0.5,
                        0.5,
                        1,
                    );

                    lr.dbloss
                }
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(400.0);

        let temporary_link_budget = 49.0 + 20.0 - dbloss;
        if temporary_link_budget > -90.0 {
            //println!("Link budget: {}", temporary_link_budget);
            let color = ramp(&temporary_link_budget);
            let base = ((((TILE_SIZE - 1) - *y) as usize * 4 * TILE_SIZE as usize)
                + ((*x) as usize * 4)) as usize;
            image_data[base] = color.0;
            image_data[base + 1] = color.1;
            image_data[base + 2] = color.2;
            image_data[base + 3] = color.3;
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

fn ramp(rssi: &f64) -> (u8, u8, u8, u8) {
    let rssi = f64::min(0.0, *rssi);
    let rrsi_abs = rssi.abs() as u8;
    //println!("{} .. {}", rssi, rrsi_abs);

    if rrsi_abs < 50 {
        (0, 255, 0, 128)
    } else if rrsi_abs < 55 {
        (64, 255, 0, 128)
    } else if rrsi_abs < 59 {
        (255, 255, 0, 128)
    } else if rrsi_abs < 63 {
        (0, 255, 0, 128)
    } else if rrsi_abs < 68 {
        (255, 255, 0, 64)
    } else if rrsi_abs < 75 {
        (255, 0, 0, 32)
    } else {
        (255, 0, 255, 0)
    }

    //COLOR_RAMP[rrsi_abs as usize - 55]
    //(255 - (rrsi_abs * 2), 0, 0)
}
