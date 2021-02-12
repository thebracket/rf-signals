use crate::c::SUIpathLoss;

pub fn sui_path_loss(
    freq_mhz: f64,
    tx_height_m: f64,
    rx_height_m: f64,
    distance_m: f64,
    mode: i32,
) -> f64 {
    unsafe {
        SUIpathLoss(
            freq_mhz,
            tx_height_m,
            rx_height_m,
            distance_m * 1000.0,
            mode,
        )
    }
}