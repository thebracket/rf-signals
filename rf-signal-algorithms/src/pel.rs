
/*
Plane Earth Loss model 
Frequency: N/A
Distance (km): Any
*/
// Plane earth loss is independent of frequency.
pub fn plane_earth_path_loss(
    tx_height_m: f64,
    rx_height_m: f64,
    distance_m: f64,
) -> f64 {
    let d = distance_m / 1000.0; // Km
    (40.0 * d.log10()) + (20.0 * tx_height_m.log10()) + (20.0 * rx_height_m.log10())
}

#[cfg(test)]
mod test {
    use super::plane_earth_path_loss;
    use float_cmp::approx_eq;
    use crate::c::PlaneEarthLoss;

    #[test]
    fn test_c_port_of_pel() {
        for tx in 1..20 {
            for rx in 1..20 {
                for d in 1..10 {
                    let native = plane_earth_path_loss(tx as f64, rx as f64, d as f64);
                    let c = unsafe { PlaneEarthLoss(d as f32 / 1000.0, tx as f32, rx as f32) };
                    //println!("tx {}, rx {}, d {}. Native: {}, C: {}", tx, rx, d, native, c);
                    assert!(approx_eq!(
                        f32,
                        native as f32,
                        c as f32,
                        ulps = 1
                    ));
                }
            }
        }
    }
}