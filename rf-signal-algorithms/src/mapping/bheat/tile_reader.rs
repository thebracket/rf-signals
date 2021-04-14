const ROW_SIZE: usize = 768;
const COL_SIZE: usize = 768;

pub struct MapTile {}

impl MapTile {
    pub fn index(lat: f64, lon: f64) -> usize {
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

    pub fn get_tile_name(lat: f64, lon: f64, heat_path: &str) -> String {
        let lat_c = if lat < 0.0 { 'S' } else { 'N' };
        let lon_c = if lon < 0.0 { 'W' } else { 'E' };

        let lat_abs = lat.abs();
        let lon_abs = lon.abs();
        let lat_floor = lat_abs.floor();
        let lon_floor = lon_abs.floor();

        let sub_lat = (lat_abs.fract() * 100.0).floor();
        let sub_lon = (lon_abs.fract() * 100.0).floor();

        format!(
            "{}/{}{:03}t{:02}_{}{:03 }t{:02}.bheat",
            heat_path, lat_c, lat_floor as i32, sub_lat, lon_c, lon_floor as i32, sub_lon,
        )
    }
}
