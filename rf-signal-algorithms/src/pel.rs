use crate::c::PlaneEarthLoss;

pub fn plane_earth_path_loss(
    tx_height_m: f32,
    rx_height_m: f32,
    distance_m: f32,
) -> f64 {
    unsafe {
        PlaneEarthLoss(
            tx_height_m,
            rx_height_m,
            distance_m * 1000.0,
        )
    }
}