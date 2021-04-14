use rf_signal_algorithms::srtm::get_altitude;
use rf_signal_algorithms::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const ROW_SIZE: usize = 768;
const COL_SIZE: usize = 768;
const NUM_CELLS: usize = COL_SIZE * ROW_SIZE;
const TOTAL_ENTRIES: usize = NUM_CELLS * 2;

pub struct MapTile {
    filename: String,
    heights: Vec<u16>,
}

impl MapTile {
    pub fn get_tile(lat: f64, lon: f64) -> Self {
        let filename = MapTile::get_tile_name(lat, lon);
        if Path::new(&filename).exists() {
            println!("Loading existing tile area: {}", filename);
            let mut tile = MapTile {
                filename,
                heights: vec![0; TOTAL_ENTRIES],
            };
            // Load the tile
            let mut file = File::open(&tile.filename).expect("Unable to open file");
            let size_of_data = TOTAL_ENTRIES;
            let mut h = [0u8; 2];
            for i in 0..size_of_data {
                let bytes_read = file.read(&mut h).expect("Read fail");
                if bytes_read != 2 {
                    panic!("Overread");
                }
                let h = bytemuck::from_bytes::<u16>(&h);
                tile.heights[i] = *h;
            }
            tile
        } else {
            // Make a new one
            let mut tile = MapTile {
                filename,
                heights: vec![0; TOTAL_ENTRIES],
            };
            for idx in 0..(NUM_CELLS) {
                let (plat, plon) = tile.coords(idx, lat, lon);
                let ll = LatLon::new(plat, plon);
                let h = get_altitude(&ll, "/home/herbert/lidarserver/terrain")
                    .unwrap_or(Distance::with_meters(0))
                    .as_meters();
                tile.heights[idx * 2] = (h * 10.0) as u16;
            }
            tile.save();
            tile
        }
    }

    pub fn index(&self, lat: f64, lon: f64) -> usize {
        let lat_abs = lat.abs();
        let lon_abs = lon.abs();
        let lat_floor = lat_abs.floor();
        let lon_floor = lon_abs.floor();

        let sub_lat = (lat_abs.fract() * 100.0).floor();
        let sub_lon = (lon_abs.fract() * 100.0).floor();

        let base_lat = lat_floor + (sub_lat / 100.0);
        let lat_min = (lat_abs - base_lat) * 100.0;
        let row_index = (lat_min * ROW_SIZE as f64) as usize;

        let base_lon = lon_floor + (sub_lon / 100.0);
        let lon_min = (lon_abs - base_lon) * 100.0;
        let col_index = (lon_min * COL_SIZE as f64) as usize;

        (row_index * COL_SIZE) + col_index
    }

    pub fn store_ground(&mut self, index: usize, altitude: u16) {
        if self.heights[index * 2] < altitude {
            self.heights[index * 2] = altitude;
        }
    }

    pub fn store_clutter(&mut self, index: usize, altitude: u16) {
        if self.heights[(index * 2) + 1] < altitude {
            self.heights[(index * 2) + 1] = altitude;
        }
    }

    pub fn save(&self) {
        let filename = format!("{}", self.filename);
        let mut file = File::create(&filename).expect("Creating file failed");
        file.write_all(bytemuck::cast_slice(&self.heights))
            .expect("Write failed");
    }

    pub fn get_tile_name(lat: f64, lon: f64) -> String {
        let lat_c = if lat < 0.0 { 'S' } else { 'N' };
        let lon_c = if lon < 0.0 { 'W' } else { 'E' };

        let lat_abs = lat.abs();
        let lon_abs = lon.abs();
        let lat_floor = lat_abs.floor();
        let lon_floor = lon_abs.floor();

        let sub_lat = (lat_abs.fract() * 100.0).floor();
        let sub_lon = (lon_abs.fract() * 100.0).floor();

        format!(
            "/home/herbert/bheat/{}{:03}t{:02}_{}{:03 }t{:02}.bheat",
            lat_c, lat_floor as i32, sub_lat, lon_c, lon_floor as i32, sub_lon,
        )
    }

    pub fn save_png(&self) {
        let t_max_height_m = self.heights.iter().step_by(2).max().unwrap_or(&0) / 10;
        let t_min_height_m = self.heights.iter().step_by(2).min().unwrap_or(&0) / 10;
        let c_max_height_m = self.heights.iter().skip(1).step_by(2).max().unwrap_or(&0) / 10;
        let c_min_height_m = self.heights.iter().skip(1).step_by(2).min().unwrap_or(&0) / 10;
        let t_span = t_max_height_m - t_min_height_m;
        let c_span = c_max_height_m - c_min_height_m;

        let filename = format!("{}.png", self.filename);
        let mut imgbuf = image::ImageBuffer::new(COL_SIZE as u32, ROW_SIZE as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let index = (y as usize * ROW_SIZE) + x as usize;
            let h = self.heights[index * 2] / 10;
            let h2 = self.heights[(index * 2) + 1] / 10;
            let shade_f = (h - t_min_height_m) as f32 / t_span as f32;
            let mut shade = if h > 0 { (255.0 * shade_f) as u8 } else { 0 };
            let shade2_f = (h2 - c_min_height_m) as f32 / c_span as f32;
            let shade2 = if h2 > 0 { (255.0 * shade2_f) as u8 } else { 0 };

            shade = u8::max(shade, shade2);

            *pixel = image::Rgb([shade, shade, shade]);
        }
        imgbuf.save(&filename).expect("Save PNG failed");
    }

    pub fn coords(&self, index: usize, lat: f64, lon: f64) -> (f64, f64) {
        let col = index % COL_SIZE;
        let row = index / COL_SIZE;

        let lat_abs = lat.abs();
        let lon_abs = lon.abs();
        let lat_floor = lat_abs.floor();
        let lon_floor = lon_abs.floor();

        let sub_lat = (lat_abs.fract() * 100.0).floor();
        let sub_lon = (lon_abs.fract() * 100.0).floor();

        let base_lat = lat_floor + (sub_lat / 100.0);
        let lat_step = (1.0 / COL_SIZE as f64) / 100.0;
        let lat_pos = base_lat + (lat_step * row as f64);

        let base_lon = lon_floor + (sub_lon / 100.0);
        let lon_step = (1.0 / COL_SIZE as f64) / 100.0;
        let lon_pos = base_lon + (lon_step * col as f64);

        let lat_final = if lat < 0.0 { lat_pos * -1.0 } else { lat_pos };

        let lon_final = if lon < 0.0 { lon_pos * -1.0 } else { lon_pos };

        (lat_final, lon_final)
    }
}
