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
    mode: EstimateMode,
) -> f64 {
    let f = frequency.as_mhz();
    let TxH = tx_height.as_meters();
    let RxH = rx_height.as_meters();
    let d = distance.as_km();
    let mode = mode.to_mode();

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
