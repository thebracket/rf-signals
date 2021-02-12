use std::f64::NAN;

use crate::{c::EgliPathLoss, Distance, Frequency};

pub fn egli_path_loss_c(freq_mhz: f32, tx_height_m: f32, rx_height_m: f32, distance_m: f32) -> f64 {
    unsafe { EgliPathLoss(freq_mhz, tx_height_m, rx_height_m, distance_m * 1000.0) }
}

/*
Frequency 30 to 1000MHz
h1 = 1m and above
h2 = 1m and above
Distance 1 to 50km
http://people.seas.harvard.edu/~jones/es151/prop_models/propagation.html#pel
*/
pub fn egli_path_loss(
    frequency: Frequency,
    tx_height: Distance,
    rx_height: Distance,
    distance: Distance,
) -> f64 {
    let f = frequency.as_mhz();
    let h1 = tx_height.as_meters();
    let h2 = rx_height.as_meters();
    let d = distance.as_km();

    let mut Lp50 = f64::NAN;
    let C1;
    let C2;

    if h1 > 10.0 && h2 > 10.0 {
        Lp50 = 85.9;
        C1 = 2.0;
        C2 = 2.0;
    } else if h1 > 10.0 {
        Lp50 = 76.3;
        C1 = 2.0;
        C2 = 1.0;
    } else if h2 > 10.0 {
        Lp50 = 76.3;
        C1 = 1.0;
        C2 = 2.0;
    } else
    // both antenna heights below 10 metres
    {
        Lp50 = 66.7;
        C1 = 1.0;
        C2 = 1.0;
    }

    Lp50 += 4.0 * _10log10f(d) + 2.0 * _10log10f(f) - C1 * _10log10f(h1) - C2 * _10log10f(h2);

    Lp50
}

#[inline(always)]
fn _10log10f(x: f64) -> f64 {
    4.342944 * x.ln()
}

#[cfg(test)]
mod test {
    use crate::{c::EgliPathLoss, egli_path_loss, Distance, Frequency};
    use float_cmp::approx_eq;

    #[test]
    fn test_c_port() {
        for f in 30..10000 {
            for tx in 1..20 {
                for rx in 1..20 {
                    for d in 1..50 {
                        let f = Frequency::with_mhz(f);
                        let tx = Distance::with_meters(tx);
                        let rx = Distance::with_meters(rx);
                        let d = Distance::with_kilometers(d);

                        let c = unsafe {
                            EgliPathLoss(
                                f.as_mhz() as f32,
                                tx.as_meters() as f32,
                                rx.as_meters() as f32,
                                d.as_km() as f32,
                            )
                        };
                        let r = egli_path_loss(f, tx, rx, d);
                        //println!("C={}, R={}", c, r);
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
