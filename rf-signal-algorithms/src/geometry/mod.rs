use crate::{Distance, LatLon};

pub fn haversine_distance(src: &LatLon, dst: &LatLon) -> Distance {
    use geo::algorithm::haversine_distance::HaversineDistance;
    Distance::with_meters(src.to_point().haversine_distance(&dst.to_point()))
}

pub fn haversine_intermediate(src: &LatLon, dst: &LatLon, extent: f64) -> LatLon {
    use geo::algorithm::haversine_intermediate::HaversineIntermediate;
    let start = src.to_radians_point();
    let end = dst.to_radians_point();
    let hi = start.haversine_intermediate(&end, extent);
    LatLon::from_radians_point(&hi)
}
