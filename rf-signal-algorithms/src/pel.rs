
/*
Plane Earth Loss model 
Frequency: N/A
Distance (km): Any
*/
// Plane earth loss is independent of frequency.
pub fn plane_earth_path_loss(
    tx_height_m: f64,
    rx_height_m: f64,
    distance_m: f64,
) -> f64 {
    let d = distance_m / 1000.0; // Km
    (40.0 * d.log10()) + (20.0 * tx_height_m.log10()) + (20.0 * rx_height_m.log10())
}
