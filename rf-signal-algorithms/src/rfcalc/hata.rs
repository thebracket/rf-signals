use crate::{Distance, EstimateMode, Frequency};

#[derive(Debug, PartialEq)]
pub enum HataError {
    FrequencyOutOfRange,
    TxHeightOutOfRange,
    RxHeightOutOfRange,
    DistanceOutOfRange,
}

/// HATA path loss estimation
/// Original: https://github.com/Cloud-RF/Signal-Server/blob/master/models/hata.cc
/// Frequency must be from 150 to 1500Mhz
/// TX Height must be from 30-200m
/// RX Height must be from 1-10m
/// Distance must be from 1-20km
pub fn hata_path_loss(
    frequency: Frequency,
    tx_height: Distance,
    rx_height: Distance,
    distance: Distance,
    mode: EstimateMode,
) -> Result<f64, HataError> {
    let mode = mode.to_mode();
    let f = frequency.as_mhz();
    if f < 150.0 || f > 1500.0 {
        return Err(HataError::FrequencyOutOfRange);
    }
    let h_b = tx_height.as_meters();
    if h_b < 30.0 || h_b > 200.0 {
        return Err(HataError::TxHeightOutOfRange);
    }
    let h_m = rx_height.as_meters();
    if h_m < 1.0 || h_m > 10.0 {
        return Err(HataError::RxHeightOutOfRange);
    }
    let d = distance.as_km();
    if d < 1.0 || d > 20.0 {
        return Err(HataError::DistanceOutOfRange);
    }

    let lh_m;
    let c_h;
    let logf = f.log10();

    if f < 200.0 {
        lh_m = (1.54 * h_m).log10();
        c_h = 8.29 * (lh_m * lh_m) - 1.1;
    } else {
        lh_m = (11.75 * h_m).log10();
        c_h = 3.2 * (lh_m * lh_m) - 4.97;
    }

    let l_u =
        69.55 + 26.16 * logf - 13.82 * h_b.log10() - c_h + (44.9 - 6.55 * h_b.log10()) * d.log10();

    if mode == 0 || mode == 1 {
        return Ok(l_u); //URBAN
    }

    if mode == 2 {
        //SUBURBAN
        let logf_28 = (f / 28.0).log10();
        return Ok(l_u - 2.0 * logf_28 * logf_28 - 5.4);
    }

    if mode == 3 {
        //OPEN
        return Ok(l_u - 4.78 * logf * logf + 18.33 * logf - 40.94);
    }

    Ok(0.0)
}
