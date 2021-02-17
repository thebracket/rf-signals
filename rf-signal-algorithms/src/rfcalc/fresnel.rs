/// Calculates the Fresnel radius of a connection.
/// Most of the time, you are interested in 60% of this number (0.6) in wireless links.
/// d1_meters is the distance from the start, d2_meters is the distance from the end.
pub fn fresnel_radius(d1_meters: f64, d2_meters: f64, freq_mhz: f64) -> f64 {
    17.31 * ((d1_meters * d2_meters) / (freq_mhz * (d1_meters + d2_meters))).sqrt()
}

#[cfg(test)]
mod test {
    use super::fresnel_radius;

    #[test]
    fn quick_fresnel_test() {
        assert!(float_cmp::approx_eq!(
            f64,
            fresnel_radius(1000.0, 1000.0, 2437.0),
            7.8406903990353305,
            ulps = 2
        ));
    }
}
