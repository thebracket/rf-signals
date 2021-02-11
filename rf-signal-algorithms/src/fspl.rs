/// Calculate the pure free-space path loss in dB of a signal and distance.
pub fn free_space_path_loss_db(frequency_mhz: f64, distance_meters: f64) -> f64 {
    let d = distance_meters / 1000.0;
    32.44 + (20.0 * frequency_mhz.log10()) + (20.0 * d.log10())
}

#[cfg(test)]
mod test {
    use super::free_space_path_loss_db;

    #[test]
    fn test_fpl() {
        assert!(float_cmp::approx_eq!(
            f64,
            free_space_path_loss_db(5800.0, 10_000.0),
            127.70855987125874,
            ulps = 2
        ));
    }
}
