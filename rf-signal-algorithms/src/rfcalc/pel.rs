use crate::Distance;

/// Plane Earth Model
/// Original C implementation: https://github.com/Cloud-RF/Signal-Server/blob/master/models/pel.cc
/// Taken from "Antennas and Propagation for wireless communication systems"  *
/// ISBN 978-0-470-84879-1 (by Alex Farrant)
///
/// Distance (meters) is unbounded.
/// Frequency is not used in this calculation.
/// Transmitter height and receiver height are height AMSL.
pub fn plane_earth_path_loss(tx_height: Distance, rx_height: Distance, distance: Distance) -> f64 {
    let d = distance.as_km();
    let txh = tx_height.as_meters();
    let rxh = rx_height.as_meters();
    (40.0 * d.log10()) + (20.0 * txh.log10()) + (20.0 * rxh.log10())
}
