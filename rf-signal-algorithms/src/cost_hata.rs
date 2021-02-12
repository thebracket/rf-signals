use crate::{Distance, EstimateMode, Frequency};

/*
COST231 extension to HATA model
Frequency 1500 to 2000MHz
TxH = Base station height 30 to 200m
RxH = Mobile station height 1 to 10m
Distance 1-20km
modes 1 = URBAN, 2 = SUBURBAN, 3 = OPEN
http://morse.colorado.edu/~tlen5510/text/classwebch3.html
*/
pub fn cost_path_loss(
    frequency: Frequency,
    tx_height: Distance,
    rx_height: Distance,
    distance: Distance,
    mode: i32,
) -> f64 {
    let f = frequency.as_mhz();
    let TxH = tx_height.as_meters();
    let RxH = rx_height.as_meters();
    let d = distance.as_km();
    //let mode = mode.to_mode();

    let mut C = 3.0; // 3dB for Urban
    let mut lRxH = (11.75 * RxH).log10();
    let mut C_H = 3.2 * (lRxH * lRxH) - 4.97; // Large city (conservative)
    let mut c0 = (69.55f64).floor(); // Note: used .floor() here because for some reason the original assigns a double to an int.
    let mut cf = (26.16f64).floor();
    if f > 1500.0 {
        c0 = 46.3;
        cf = 33.9;
    }
    if mode == 2 {
        C = 0.0; // Medium city (average)
        lRxH = (1.54 * RxH).log10();
        C_H = 8.29 * (lRxH * lRxH) - 1.1;
    }
    if mode == 3 {
        C = -3.0; // Small city (Optimistic)
        C_H = (1.1 * f.log10() - 0.7) * RxH - (1.56 * f.log10()) + 0.8;
    }
    let logf = f.log10();
    let dbloss = c0 + (cf * logf) - (13.82 * TxH.log10()) - C_H
        + (44.9 - 6.55 * TxH.log10()) * d.log10()
        + C;
    return dbloss;
}

#[cfg(test)]
mod test {
    use crate::{c::COST231pathLoss, cost_path_loss, Distance, Frequency};
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
                                COST231pathLoss(
                                    f.as_mhz() as f32,
                                    tx.as_meters() as f32,
                                    rx.as_meters() as f32,
                                    d.as_km() as f32,
                                    mode,
                                )
                            };

                            let r = cost_path_loss(f, tx, rx, d, mode);

                            assert!(
                                approx_eq!(f32, c as f32, r as f32, ulps = 4),
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
