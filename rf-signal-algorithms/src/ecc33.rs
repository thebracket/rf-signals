use crate::{Distance, EstimateMode, Frequency};

#[derive(Debug, PartialEq)]
pub enum ECC33Error {
    FrequencyOutOfRange,
    HeightOfOutRange,
    DistanceOutOfRange,
}

/// ECC33 Path Loss Estimation
/// Original: https://github.com/Cloud-RF/Signal-Server/blob/master/models/ecc33.cc
/// Frequency must be 30..1,000 Mhz
/// Heights must be >1m
/// Distance must be 1..50km
pub fn ecc33_path_loss(
    frequency: Frequency,
    tx_height: Distance,
    rx_height: Distance,
    distance: Distance,
    mode: EstimateMode,
) -> Result<f64, ECC33Error> {
    let tx_h = tx_height.as_meters();
    let mut rx_h = rx_height.as_meters();
    if tx_h < 1.0 || rx_h < 1.0 {
        return Err(ECC33Error::HeightOfOutRange);
    }
    let d = distance.as_km();
    if d < 1.0 || d > 50.0 {
        return Err(ECC33Error::DistanceOutOfRange);
    }
    let f = frequency.as_ghz();
    if f < 0.03 || f > 1.0 {
        return Err(ECC33Error::FrequencyOutOfRange);
    }
    let mode = mode.to_mode();

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

    Ok(afs + abm - gb - gr)
}
