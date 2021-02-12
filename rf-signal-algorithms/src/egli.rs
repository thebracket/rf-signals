use crate::c::EgliPathLoss;

pub fn egli_path_loss(freq_mhz: f32, tx_height_m: f32, rx_height_m: f32, distance_m: f32) -> f64 {
    unsafe { EgliPathLoss(freq_mhz, tx_height_m, rx_height_m, distance_m * 1000.0) }
}
