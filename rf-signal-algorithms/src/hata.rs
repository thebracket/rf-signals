use crate::{Distance, Frequency};

/*
HATA URBAN model for cellular planning
Frequency (MHz) 150 to 1500MHz
Base station height 30-200m
Mobile station height 1-10m
Distance 1-20km

mode 1 = URBAN
mode 2 = SUBURBAN
mode 3 = OPEN
*/

pub fn hata_path_loss(
    frequency: Frequency,
    tx_height: Distance,
    rx_height: Distance,
    distance: Distance,
    mode: i32
) -> f64 {
    let f = frequency.as_mhz();
    let h_b = tx_height.as_meters();
    let h_m = rx_height.as_meters();
    let d = distance.as_km();

    let lh_m;
    let c_h;
    let logf = f.log10();

    if f<200.0 {
		lh_m = (1.54 * h_m).log10();
		c_h = 8.29 * (lh_m * lh_m) - 1.1;
	} else {
		lh_m = (11.75 * h_m).log10();
		c_h = 3.2 * (lh_m * lh_m) - 4.97;
	}

    let l_u = 69.55 + 26.16 * logf - 13.82 * h_b.log10() - c_h + (44.9 - 6.55 * h_b.log10()) * d.log10();

    if mode==0 || mode == 1 {
		return l_u;	//URBAN
	}

	if mode == 2 {	//SUBURBAN
		let logf_28 = (f / 28.0).log10();
		return l_u - 2.0 * logf_28 * logf_28 - 5.4;
	}

	if mode == 3 {	//OPEN
		return l_u - 4.78 * logf * logf + 18.33 * logf - 40.94;
	}

    0.0
}

#[cfg(test)]
mod test {
    use crate::{Distance, Frequency, c::HATApathLoss, hata_path_loss};
    use float_cmp::approx_eq;

    #[test]
    fn c_port_test() {
        for freq in 150 .. 1500 {
            for tx in 30..200 {
                for rx in 1..10 {
                    for distance in 1..20 {
                        for mode in 1..=3 {
                            let f = Frequency::with_mhz(freq);
                            let txh = Distance::with_meters(tx);
                            let rxh = Distance::with_meters(rx);
                            let d = Distance::with_kilometers(distance);
                            let c = unsafe { HATApathLoss(f.as_mhz() as f32, txh.as_meters() as f32, rxh.as_meters() as f32, d.as_km() as f32, mode) };
                            let r = hata_path_loss(f, txh, rxh, d, mode);
                            assert!(approx_eq!(
                                f32,
                                c as f32,
                                r as f32,
                                ulps = 3
                            ), "mode={}, f={}, d={}, C={}, R={}", mode, freq, distance, c, r);
                        }
                    }
                }
            }
        }
    }
}