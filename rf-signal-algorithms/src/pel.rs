use crate::Distance;

/// Plane Earth Model
/// Original C implementation: https://github.com/Cloud-RF/Signal-Server/blob/master/models/pel.cc
/// Taken from "Antennas and Propagation for wireless communication systems"  *
/// ISBN 978-0-470-84879-1 (by Alex Farrant)
///
/// Distance (meters) is unbounded.
/// Frequency is not used in this calculation.
/// Transmitter height and receiver height are height AMSL.
pub fn plane_earth_path_loss(
    tx_height_m: f64,
    rx_height_m: f64,
    distance: Distance,
) -> f64 {
    let d = distance.as_km();
    (40.0 * d.log10()) + (20.0 * tx_height_m.log10()) + (20.0 * rx_height_m.log10())
}
