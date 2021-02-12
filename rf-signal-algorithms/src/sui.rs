use crate::{Distance, EstimateMode, Frequency};
use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
pub enum SuiError {
    FrequencyOutOfRange,
}

/// Calculates SUI path loss.
/// Original: https://github.com/Cloud-RF/Signal-Server/blob/master/models/sui.cc
/// Frequency must be in 1900 to 11,000 Mhz range.
/// Transmitter height, receiver_height and distance are unbounded.
/// See http://www.cl.cam.ac.uk/research/dtg/lce-pub/public/vsa23/VTC05_Empirical.pdf
/// And https://mentor.ieee.org/802.19/file/08/19-08-0010-00-0000-sui-path-loss-model.doc
pub fn sui_path_loss(
    frequency: Frequency,
    tx_height: Distance,
    rx_height: Distance,
    distance: Distance,
    mode: EstimateMode,
) -> Result<f64, SuiError> {
    let mode = mode.to_mode();
    let d = distance.as_meters();
    let f = frequency.as_mhz();
    if f < 1900.0 || f > 11000.0 {
        return Err(SuiError::FrequencyOutOfRange);
    }
    let txh = tx_height.as_meters();
    let rxh = rx_height.as_meters();
    let mut a = 4.6;
    let mut b = 0.0075;
    let mut c = 12.6;
    let s = 8.2;
    let mut xhcf = -10.8;

    if mode == 2 {
        // Suburban
        a = 4.0;
        b = 0.0065;
        c = 17.1;
        xhcf = -10.8;
    }
    if mode == 3 {
        // Rural
        a = 3.6;
        b = 0.005;
        c = 20.0;
        xhcf = -20.0;
    }

    let d0 = 100.0;
    let big_a = 20.0 * ((4.0 * PI * d0) / (300.0 / f)).log10();
    let y = a - (b * txh) + (c / txh);
    let mut xf = 0.0;
    let mut xh = 0.0;

    if f > 2000.0 {
        xf = 6.0 * (f / 2.0).log10();
        xh = xhcf * (rxh / 2.0).log10();
    }

    Ok(big_a + (10.0 * y) * (d / d0).log10() + xf + xh + s)
}
