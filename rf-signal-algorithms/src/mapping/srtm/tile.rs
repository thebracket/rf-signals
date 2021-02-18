use crate::LatLon;
use std::path::Path;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Denotes the types of SRTM (.hgt) tile formats that are available.
pub enum SrtmTile {
    Srtm1 {
        lat: i16,
        lon: i16,
    },
    Srtm3 {
        lat: i16,
        lon: i16,
    },
    SrtmThird {
        lat: i16,
        lon: i16,
        lat_tile: u8,
        lon_tile: u8,
    },
}

impl SrtmTile {
    /// Determine if a tile is available, and return available strategy
    /// for retrieving it at the best possible resolution.
    pub fn check_availability(loc: &LatLon, terrain_path: &str) -> Option<Self> {
        let third = loc.to_srtm_third();
        if Path::new(&third.filename(terrain_path)).exists() {
            return Some(third);
        }

        let three = loc.to_srtm3();
        if Path::new(&three.filename(terrain_path)).exists() {
            return Some(three);
        }

        let one = loc.to_srtm1();
        if Path::new(&one.filename(terrain_path)).exists() {
            return Some(one);
        }

        None
    }

    /// Calculates an SRTM filename based on an SrtmTile entry.
    /// terrain_path denotes a directory prefix to attach.
    pub fn filename(&self, terrain_path: &str) -> String {
        match self {
            SrtmTile::Srtm1 { lat, lon } => format!(
                "{}/1/{}{:02}{}{:03}.hgt",
                terrain_path,
                if *lat >= 0 { 'N' } else { 'S' },
                lat.abs(),
                if *lon >= 0 { 'E' } else { 'W' },
                lon.abs()
            ),
            SrtmTile::Srtm3 { lat, lon } => format!(
                "{}/3/{}{:02}{}{:03}.hgt",
                terrain_path,
                if *lat >= 0 { 'N' } else { 'S' },
                lat.abs(),
                if *lon >= 0 { 'E' } else { 'W' },
                lon.abs()
            ),
            SrtmTile::SrtmThird {
                lat,
                lon,
                lat_tile,
                lon_tile,
            } => format!(
                "{}/third/{}{:02}{}{:03}T{:01}{:01}.hgt",
                terrain_path,
                if *lat >= 0 { 'N' } else { 'S' },
                lat.abs(),
                if *lon >= 0 { 'E' } else { 'W' },
                lon.abs(),
                lat_tile,
                lon_tile
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srtm1_filename() {
        let tile = SrtmTile::Srtm1 { lat: 38, lon: -93 };
        let desired = "/1/N38W093.hgt";
        assert_eq!(desired, tile.filename(""));
    }

    #[test]
    fn test_srtm3_filename() {
        let tile = SrtmTile::Srtm3 { lat: 38, lon: -93 };
        let desired = "/3/N38W093.hgt";
        assert_eq!(desired, tile.filename(""));
    }

    #[test]
    fn test_srtm_third_filename() {
        let tile = SrtmTile::SrtmThird {
            lat: 38,
            lon: -93,
            lat_tile: 9,
            lon_tile: 7,
        };
        let desired = "/third/N38W093T97.hgt";
        assert_eq!(desired, tile.filename(""));
    }
}
