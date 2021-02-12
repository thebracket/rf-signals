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


#[cfg(test)]
mod test {
    use super::*;
    use crate::c::SoilPathLoss;
    use float_cmp::approx_eq;

    #[test]
    fn compare_c_with_rust() {
        for p in 1..=15 {
            for f in 1..=10 {
                for d in 1..=10 {
                    let freq = Frequency::with_mhz(f as f32 * 1000.0);
                    let d = Distance::with_kilometers(d);

                    let c = unsafe { SoilPathLoss(freq.as_mhz() as f32, d.as_km() as f32, p as f32) };
                    let r = soil_path_loss(freq, d, p as f64);
                    assert!(approx_eq!(
                        f32,
                        c as f32,
                        r as f32,
                        ulps = 2
                    ));
                }
            }
        }
    }
}