use crate::{Distance, Frequency};

#[derive(Debug, PartialEq)]
pub enum SoilError {
    PermittivityOutOfRange
}

/// Soil Path Loss formula.
/// Original: https://github.com/Cloud-RF/Signal-Server/blob/master/models/soil.cc
/// Permitivity must be from 1 to 15, 1 = Worst, 15 = Best.
/// Distance and Frequency are unbounded.
pub fn soil_path_loss(frequency: Frequency, distance: Distance, terrain_permittivity: f64) -> Result<f64, SoilError> {
    if terrain_permittivity < 1.0 || terrain_permittivity > 15.0 {
        return Err(SoilError::PermittivityOutOfRange);
    }
    let d = distance.as_km();
    let f = frequency.as_mhz();
    let soil = 120.0 / terrain_permittivity;
    Ok(6.4 + ( 20.0 * d.log10() ) + ( 20.0 * f.log10() ) + ( 8.69 * soil ))
}
