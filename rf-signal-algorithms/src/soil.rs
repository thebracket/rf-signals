use crate::{Distance, Frequency};

/*
* Frequency: Any MHz
* Distance: Any Km
* Terrain permittivity: 1 - 15 (Bad to Good)
*/
pub fn soil_path_loss(frequency: Frequency, distance: Distance, terrain_permittivity: f64) -> f64 {
    let d = distance.as_km();
    let f = frequency.as_mhz();
    let soil = 120.0 / terrain_permittivity;
    6.4 + ( 20.0 * d.log10() ) + ( 20.0 * f.log10() ) + ( 8.69 * soil )
}
