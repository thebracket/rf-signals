use super::{Frequency, Distance};

/// Calculate the pure free-space path loss in dB of a signal and distance.
pub fn free_space_path_loss_db(frequency: Frequency, distance: Distance) -> f64 {
    let d = distance.as_km();
    let f = frequency.as_mhz();
    32.44 + (20.0 * f.log10()) + (20.0 * d.log10())
}

#[cfg(test)]
mod test {
    use super::{free_space_path_loss_db, Frequency, Distance};

    #[test]
    fn test_fpl() {
        assert!(float_cmp::approx_eq!(
            f64,
            free_space_path_loss_db(Frequency::with_ghz(5.8), Distance::with_kilometers(10)),
            127.70855987125874,
            ulps = 2
        ));
    }
}
