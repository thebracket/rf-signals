use crate::{Distance, LatLon};
use geo::{Distance as _, Haversine, InterpolatePoint};

pub fn haversine_distance(src: &LatLon, dst: &LatLon) -> Distance {
    Distance::with_meters(Haversine.distance(src.to_point(), dst.to_point()))
}

pub fn haversine_intermediate(src: &LatLon, dst: &LatLon, extent: f64) -> LatLon {
    LatLon::from(Haversine.point_at_ratio_between(src.to_point(), dst.to_point(), extent))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn haversine_distance_is_in_meters() {
        let distance = haversine_distance(&LatLon::new(0.0, 0.0), &LatLon::new(0.0, 1.0));
        assert!((distance.as_meters() - 111_195.08).abs() < 1.0);
    }

    #[test]
    fn haversine_intermediate_finds_the_midpoint() {
        let midpoint = haversine_intermediate(&LatLon::new(0.0, 0.0), &LatLon::new(0.0, 1.0), 0.5);
        assert!(midpoint.lat().abs() < 1.0e-9);
        assert!((midpoint.lon() - 0.5).abs() < 1.0e-9);
    }
}
