use std::f64::consts::PI;
use crate::{Distance, Frequency, c::SUIpathLoss};

pub fn sui_path_loss(
    frequency: Frequency,
    tx_height: Distance,
    rx_height: Distance,
    distance: Distance,
    mode: i32,
) -> f64 {
    let d = distance.as_meters();
    let f = frequency.as_mhz();
    let txh = tx_height.as_meters();
    let rxh = rx_height.as_meters();
    let mut a = 4.6;
    let mut b = 0.0075;
    let mut c = 12.6;
    let s = 8.2;
    let mut xhcf = -10.8;

    if mode == 2 { // Suburban
        a = 4.0;
        b = 0.0065;
        c = 17.1;
        xhcf = -10.8;
    }
    if mode == 3 { // Rural
        a = 3.6;
        b = 0.005;
        c = 20.0;
        xhcf = -20.0;
    }

    let d0 = 100.0;
    let A = 20.0 * ( ( 4.0 * PI * d0 ) / ( 300.0 / f) ).log10();
    let y = a - (b * txh) + (c / txh);
    let mut Xf = 0.0;
    let mut Xh = 0.0;

    if f > 2000.0 {
        Xf = 6.0 * (f / 2.0).log10();
        Xh = xhcf * (rxh / 2.0).log10();
    }

    A + (10.0 * y) * (d / d0).log10() + Xf + Xh + s
}

#[cfg(test)]
mod test {
    use crate::{Distance, Frequency, c::SUIpathLoss, sui_path_loss};
    use float_cmp::approx_eq;

    #[test]
    fn test_c_port() {
        for f in 1900..10000 {
            for txh in 1..30 {
                for rxh in 1..30 {
                    for d in 1..20 {
                        for mode in 1..=3 {
                            let f = Frequency::with_mhz(f);
                            let tx = Distance::with_meters(txh);
                            let rx = Distance::with_meters(rxh);
                            let d = Distance::with_kilometers(d);

                            let c = unsafe { SUIpathLoss(f.as_mhz(), tx.as_meters(), rx.as_meters(), d.as_km(), mode) };
                            let r = sui_path_loss(f, tx, rx, d, mode);

                            //println!("C={}, R={}", c, r);
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
    }
}