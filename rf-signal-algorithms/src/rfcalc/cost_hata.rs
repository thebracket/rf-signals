use crate::{Distance, EstimateMode, Frequency};

#[derive(Debug, PartialEq)]
pub enum CostError {
    FrequencyOutOfRange,
    HeightOutOfRange,
    DistanceOutOfRange,
}

/// COST231 extension to HATA path loss model.
/// Original: https://github.com/Cloud-RF/Signal-Server/blob/master/models/cost.cc
/// See: http://morse.colorado.edu/~tlen5510/text/classwebch3.html
/// Frequency must be 1,500 .. 2,000 Mhz
/// tx_height must be 30..200 meters
/// rx_height must be 1..20 meters
/// Distance must be 1..20 km
pub fn cost_path_loss(
    frequency: Frequency,
    tx_height: Distance,
    rx_height: Distance,
    distance: Distance,
    mode: EstimateMode,
) -> Result<f64, CostError> {
    let f = frequency.as_mhz();
    if f < 1_500.0 || f > 2_000.0 {
        return Err(CostError::FrequencyOutOfRange);
    }
    let txh = tx_height.as_meters();
    let rxh = rx_height.as_meters();
    if txh < 30.0 || txh > 200.0 {
        return Err(CostError::HeightOutOfRange);
    }
    if rxh < 1.0 || rxh > 10.0 {
        return Err(CostError::HeightOutOfRange);
    }
    let d = distance.as_km();
    if d < 1.0 || d > 20.0 {
        return Err(CostError::DistanceOutOfRange);
    }
    let mode = mode.to_mode();

    let mut c = 3.0; // 3dB for Urban
    let mut lrxh = (11.75 * rxh).log10();
    let mut c_h = 3.2 * (lrxh * lrxh) - 4.97; // Large city (conservative)
    let mut c0 = (69.55f64).floor(); // Note: used .floor() here because for some reason the original assigns a double to an int.
    let mut cf = (26.16f64).floor();
    if f > 1500.0 {
        c0 = 46.3;
        cf = 33.9;
    }
    if mode == 2 {
        c = 0.0; // Medium city (average)
        lrxh = (1.54 * rxh).log10();
        c_h = 8.29 * (lrxh * lrxh) - 1.1;
    }
    if mode == 3 {
        c = -3.0; // Small city (Optimistic)
        c_h = (1.1 * f.log10() - 0.7) * rxh - (1.56 * f.log10()) + 0.8;
    }
    let logf = f.log10();
    let dbloss = c0 + (cf * logf) - (13.82 * txh.log10()) - c_h
        + (44.9 - 6.55 * txh.log10()) * d.log10()
        + c;
    return Ok(dbloss);
}
