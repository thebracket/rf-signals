use crate::{Distance, Frequency};

/*
Frequency 30 to 1000MHz
h1 = 1m and above
h2 = 1m and above
Distance 1 to 50km
http://people.seas.harvard.edu/~jones/es151/prop_models/propagation.html#pel
*/
pub fn ecc33_path_loss(
    frequency: Frequency,
    tx_height: Distance,
    rx_height: Distance,
    distance: Distance,
    mode: i32,
) -> f64 {
    let tx_h = tx_height.as_meters();
    let mut rx_h = rx_height.as_meters();
    let d = distance.as_km();
    let f = frequency.as_ghz();

    // Sanity check as this model operates within limited Txh/Rxh bounds
    if tx_h - rx_h < 0.0 {
        rx_h = rx_h / (d * 2.0);
    }

    let mut gr = 0.759 * rx_h - 1.862; // Big city with tall buildings (1)
    let afs = 92.4 + 20.0 * d.log10() + 20.0 * f.log10();
    let abm = 20.41 + 9.83 * d.log10() + 7.894 * f.log10() + 9.56 * (f.log10() * f.log10());
    let gb = (tx_h / 200.0).log10() * (13.958 + 5.8 * (d.log10() * d.log10()));
    if mode > 1 {
        // Medium city (Europe)
        gr = (42.57 + 13.7 * f.log10()) * (rx_h.log10() - 0.585);
    }

    return afs + abm - gb - gr;
}

#[cfg(test)]
mod test {
    use crate::{c::ECC33pathLoss, ecc33_path_loss, Distance, Frequency};
    use float_cmp::approx_eq;

    #[test]
    fn test_c_port() {
        for f in 30..1000 {
            for tx in 1..20 {
                for rx in 1..20 {
                    for d in 1..50 {
                        for mode in 1..3 {
                            let f = Frequency::with_mhz(f);
                            let tx = Distance::with_meters(tx);
                            let rx = Distance::with_meters(rx);
                            let d = Distance::with_kilometers(d);

                            let c = unsafe {
                                ECC33pathLoss(
                                    f.as_mhz() as f32,
                                    tx.as_meters() as f32,
                                    rx.as_meters() as f32,
                                    d.as_km() as f32,
                                    mode,
                                )
                            };

                            let r = ecc33_path_loss(f, tx, rx, d, mode);

                            assert!(
                                approx_eq!(f32, c as f32, r as f32, ulps = 2),
                                "C={}, R={}",
                                c,
                                r
                            );
                        }
                    }
                }
            }
        }
    }
}
