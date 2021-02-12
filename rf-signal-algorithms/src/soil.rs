use crate::c::SoilPathLoss;

pub fn soil_path_loss(
    freq_mhz: f32,
    distance_m: f32,
    terrain_permittivity: f32
) -> f64 {
    unsafe {
        SoilPathLoss(
            freq_mhz,
            distance_m * 1000.0,
            terrain_permittivity,
        )
    }
}