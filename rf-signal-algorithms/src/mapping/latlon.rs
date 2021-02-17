use geo::Point;
use super::srtm::SrtmTile;

// Represents a point on the globe
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LatLon(Point<f64>);

impl From<Point<f64>> for LatLon {
    fn from(item: Point<f64>) -> Self {
        Self(item)
    }
}

impl LatLon {
    /// Create a new lat/lon location
    pub fn new(lat: f64, lon: f64) -> Self {
        Self(Point::new(lon, lat))
    }

    /// Retrieve Latitude
    pub fn lat(&self) -> f64 {
        self.0.lat()
    }

    /// Retrieve Longitude
    pub fn lon(&self) -> f64 {
        self.0.lng()
    }

    /// Retrieve a geo point in Radians
    pub fn to_radians_point(&self) -> Point<f64> {
        const DEG_RAD: f64 = 1.74532925199e-02;
        Point::new(self.0.lat() * DEG_RAD, self.0.lng() * DEG_RAD)
    }

    /// Create from a geo point in Radians
    pub fn from_radians_point(pt: &Point<f64>) -> Self {
        const DEG_RAD: f64 = 1.74532925199e-02;
        LatLon(Point::new(pt.lat() / DEG_RAD, pt.lng() / DEG_RAD))
    }

    fn floor(&self) -> LatLon {
        LatLon::new(self.0.lat().floor(), self.0.lng().floor())
    }

    /// Calculate the SRTM tile containing this point
    pub fn to_srtm1(&self) -> SrtmTile {
        let floor = self.floor();
        SrtmTile::Srtm1 {
            lat: floor.0.lat() as i16,
            lon: floor.0.lng() as i16,
        }
    }

    /// Calculate the SRTM3 tile containing this point
    pub fn to_srtm3(&self) -> SrtmTile {
        let floor = self.floor();
        SrtmTile::Srtm3 {
            lat: floor.0.lat() as i16,
            lon: floor.0.lng() as i16,
        }
    }

    /// Calculate the SRTM-Third tile containing this point
    pub fn to_srtm_third(&self) -> SrtmTile {
        let floor = self.floor();
        let lat_extent_base_10 = self.lat() * 10.0 - floor.lat() * 10.0;
        let lat_extent_base_9 = ((lat_extent_base_10 / 10.0) * 9.0) + 1.0;
        let lat_sub_tile = lat_extent_base_9.floor();

        let lon_extent_base_10 = self.lon() * 10.0 - floor.lon() * 10.0;
        let lon_extent_base_9 = ((lon_extent_base_10 / 10.0) * 9.0) + 1.0;
        let lon_sub_tile = lon_extent_base_9.floor();

        SrtmTile::SrtmThird {
            lat: floor.lat() as i16,
            lon: floor.lon() as i16,
            lat_tile: lat_sub_tile as u8,
            lon_tile: lon_sub_tile as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srtm1_filename() {
        let loc = LatLon::new(38.947775, -92.323385);
        let srtm1 = loc.to_srtm1();
        assert_eq!(srtm1, SrtmTile::Srtm1 { lat: 38, lon: -93 });
    }

    #[test]
    fn test_srtm3_filename() {
        let loc = LatLon::new(38.947775, -92.323385);
        let srtm3 = loc.to_srtm3();
        assert_eq!(srtm3, SrtmTile::Srtm3 { lat: 38, lon: -93 });
    }

    #[test]
    fn test_srtm_third_filename() {
        let loc = LatLon::new(38.947775, -92.323385);
        let srtm3 = loc.to_srtm_third();
        assert_eq!(
            srtm3,
            SrtmTile::SrtmThird {
                lat: 38,
                lon: -93,
                lat_tile: 9,
                lon_tile: 7,
            }
        );
    }
}