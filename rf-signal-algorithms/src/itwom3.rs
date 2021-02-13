#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::c::point_to_point;

/******************************************************************************

    Note that point_to_point_two has become point_to_point
    for drop-in interface to splat.cpp.
    The new variable inputs,
    double enc_ncc_clcref,
    double clutter_height,
    double clutter_density,
    double delta_h_diff, and
    int mode_var)
    have been given fixed values below.

    pol:
        0-Horizontal, 1-Vertical, 2-Circular

    radio_climate:
        1-Equatorial, 2-Continental Subtropical,
        3-Maritime Tropical, 4-Desert, 5-Continental Temperate,
        6-Maritime Temperate, Over Land, 7-Maritime Temperate,
        Over Sea

    conf, rel: .01 to .99

    elev[]: [num points - 1], [delta dist(meters)],
            [height(meters) point 1], ..., [height(meters) point n]

    clutter_height  	25.2 meters for compatibility with ITU-R P.1546-2.

    clutter_density 	1.0 for compatibility with ITU-R P.1546-2.

    delta_h_diff		optional delta h for beyond line of sight. 90 m. average.
                setting to 0.0 will default to use of original internal
                use of delta-h for beyond line-of-sight range.

    mode_var		set to 12; or to 1 for FCC ILLR;  see documentation

    enc_ncc_clcref 		clutter refractivity; 1000 N-units to match ITU-R P.1546-2

    eno=eno_ns_surfref	atmospheric refractivity at sea level; 301 N-units nominal
                (ranges from 250 for dry, hot day to 450 on hot, humid day]
                (stabilizes near 301 in cold, clear weather)

    errnum: 0- No Error.
        1- Warning: Some parameters are nearly out of range.
                    Results should be used with caution.
        2- Note: Default parameters have been substituted for
                 impossible ones.
        3- Warning: A combination of parameters is out of range.
                Results are probably invalid.
        Other-  Warning: Some parameters are out of range.
            Results are probably invalid.

*****************************************************************************/
fn PointToPoint(
    elev: &mut [f64],
    tht_m: f64,
    rht_m: f64,
    eps_dielect: f64,
    sgm_conductivity: f64,
    eno_ns_surfref: f64,
    frq_mhz: f64,
    radio_climate: ::std::os::raw::c_int,
    pol: ::std::os::raw::c_int,
    conf: f64,
    rel: f64,
) -> PTPResult {
    use std::ffi::CStr;
    let mut dbloss = 0.0f64;
    let mut mode = [0 as std::os::raw::c_char; 128];
    let mut errnum: std::os::raw::c_int = 0;

    unsafe {
        point_to_point(
            elev.as_mut_ptr(),
            tht_m,
            rht_m,
            eps_dielect,
            sgm_conductivity,
            eno_ns_surfref,
            frq_mhz,
            radio_climate,
            pol,
            conf,
            rel,
            &mut dbloss,
            mode.as_mut_ptr(),
            &mut errnum,
        );
    }

    let mode_str = unsafe { CStr::from_ptr(mode.as_ptr()).to_string_lossy() };

    PTPResult {
        dbloss: dbloss,
        mode: mode_str.to_string(),
        error_num: errnum,
    }
}

#[derive(Debug)]
pub struct PTPResult {
    pub dbloss: f64,
    pub mode: String,
    pub error_num: i32,
}

#[derive(Debug, PartialEq)]
pub enum PTPError {
    DistanceTooShort,
    DistanceTooLong,
    AltitudeTooHigh,
    AltitudeTooLow,
}

/// Describes terrain for an IWOM Point-To-Point path.
/// Elevations should be an array of altitudes along the path, starting at the transmitter and ending at the receiver.
/// The height of transmitters and receivers is height above the elevations specified.
#[derive(Debug)]
pub struct PTPPath {
    pub elevations: Vec<f64>,
    pub transmit_height: f64,
    pub receive_height: f64,
}

impl PTPPath {
    /// Construct a new PTP path for ITWOM evaluation. The constructor takes care of pre-pending the fields
    /// used by the C algorithm.
    pub fn new(
        elevations: Vec<f64>,
        transmit_height: f64,
        receive_height: f64,
        step_size_meters: f64,
    ) -> Result<Self, PTPError> {
        let total_distance: f64 = elevations.len() as f64 * step_size_meters;
        if total_distance < 1000.0 {
            return Err(PTPError::DistanceTooShort);
        } else if total_distance > 2000000.0 {
            return Err(PTPError::DistanceTooLong);
        }

        for a in &elevations {
            if *a < 0.5 {
                return Err(PTPError::AltitudeTooLow);
            } else if *a > 3000.0 {
                return Err(PTPError::AltitudeTooHigh);
            }
        }

        let mut path = Self {
            elevations,
            transmit_height,
            receive_height,
        };

        // Index 0 is the number of elements, next up is the distance per step
        path.elevations.insert(0, step_size_meters);
        path.elevations
            .insert(0, path.elevations.len() as f64 - 3.0);

        Ok(path)
    }
}

#[derive(Debug)]
pub struct PTPClimate {
    pub eps_dialect: f64,
    pub sgm_conductivity: f64,
    pub eno_ns_surfref: f64,
    pub radio_climate: i32,
}

pub enum GroundConductivity {
    SaltWater,
    GoodGround,
    FreshWater,
    MarshyLand,
    Farmland,
    Forest,
    AverageGround,
    Mountain,
    Sand,
    City,
    PoorGround,
}

pub enum RadioClimate {
    Equatorial,
    ContinentalSubtropical,
    MaritimeSubtropical,
    Desert,
    ContinentalTemperate,
    MaritimeTemperateLand,
    MaritimeTemperateSea,
}

impl PTPClimate {
    pub fn default() -> Self {
        Self {
            eps_dialect: 15.0,
            sgm_conductivity: 0.005,
            eno_ns_surfref: 301.0,
            radio_climate: 5,
        }
    }

    pub fn new(ground: GroundConductivity, climate: RadioClimate) -> Self {
        Self {
            eps_dialect: match ground {
                GroundConductivity::SaltWater => 80.0,
                GroundConductivity::GoodGround => 25.0,
                GroundConductivity::FreshWater => 80.0,
                GroundConductivity::MarshyLand => 12.0,
                GroundConductivity::Farmland => 15.0,
                GroundConductivity::Forest => 15.0,
                GroundConductivity::AverageGround => 15.0,
                GroundConductivity::Mountain => 13.0,
                GroundConductivity::Sand => 13.0,
                GroundConductivity::City => 5.0,
                GroundConductivity::PoorGround => 4.0,
            },
            sgm_conductivity: match ground {
                GroundConductivity::SaltWater => 5.0,
                GroundConductivity::GoodGround => 0.020,
                GroundConductivity::FreshWater => 0.010,
                GroundConductivity::MarshyLand => 0.007,
                GroundConductivity::Farmland => 0.005,
                GroundConductivity::Forest => 0.005,
                GroundConductivity::AverageGround => 0.005,
                GroundConductivity::Mountain => 0.002,
                GroundConductivity::Sand => 0.002,
                GroundConductivity::City => 0.001,
                GroundConductivity::PoorGround => 0.001,
            },
            eno_ns_surfref: 301.0,
            radio_climate: match climate {
                RadioClimate::Equatorial => 1,
                RadioClimate::ContinentalSubtropical => 2,
                RadioClimate::MaritimeSubtropical => 3,
                RadioClimate::Desert => 4,
                RadioClimate::ContinentalTemperate => 5,
                RadioClimate::MaritimeTemperateLand => 6,
                RadioClimate::MaritimeTemperateSea => 7,
            },
        }
    }
}

pub fn ItwomPointToPoint(
    path: &mut PTPPath,
    climate: PTPClimate,
    frequency_mhz: f64,
    confidence: f64,
    rel: f64,
    polarity: i32,
) -> PTPResult {
    PointToPoint(
        &mut path.elevations,
        path.transmit_height,
        path.receive_height,
        climate.eps_dialect,
        climate.sgm_conductivity,
        climate.eno_ns_surfref,
        frequency_mhz,
        climate.radio_climate,
        polarity,
        confidence,
        rel,
    )
}

#[cfg(test)]
mod test {
    use float_cmp::approx_eq;

    use super::*;

    #[test]
    fn test_too_short() {
        assert_eq!(
            PTPPath::new(vec![1.0; 2], 100.0, 100.0, 10.0).err(),
            Some(PTPError::DistanceTooShort)
        );
    }

    #[test]
    fn test_too_long() {
        assert_eq!(
            PTPPath::new(vec![1.0; 2000000], 100.0, 100.0, 10.0).err(),
            Some(PTPError::DistanceTooLong)
        );
    }

    #[test]
    fn altitudes_too_low() {
        assert_eq!(
            PTPPath::new(vec![0.4; 200], 100.0, 100.0, 10.0).err(),
            Some(PTPError::AltitudeTooLow)
        );
    }

    #[test]
    fn altitudes_too_high() {
        assert_eq!(
            PTPPath::new(vec![3500.0; 200], 100.0, 100.0, 10.0).err(),
            Some(PTPError::AltitudeTooHigh)
        );
    }

    #[test]
    fn basic_fspl_test() {
        let mut terrain_path = PTPPath::new(vec![1.0; 200], 100.0, 100.0, 10.0).unwrap();

        let itwom_test = ItwomPointToPoint(
            &mut terrain_path,
            PTPClimate::default(),
            5800.0,
            0.5,
            0.5,
            1,
        );

        assert_eq!(itwom_test.mode, "L-o-S");
        assert_eq!(itwom_test.error_num, 0);
        assert!(approx_eq!(
            f64,
            itwom_test.dbloss,
            113.65156617174829,
            ulps = 2
        ));
    }

    #[test]
    fn basic_one_obstruction() {
        let mut elevations = vec![1.0; 200];
        elevations[100] = 110.0;
        let mut terrain_path = PTPPath::new(elevations, 100.0, 100.0, 10.0).unwrap();

        let itwom_test = ItwomPointToPoint(
            &mut terrain_path,
            PTPClimate::default(),
            5800.0,
            0.5,
            0.5,
            1,
        );

        assert_eq!(itwom_test.mode, "1_Hrzn_Diff");
        assert_eq!(itwom_test.error_num, 0);
    }

    #[test]
    fn basic_two_obstructions() {
        let mut elevations = vec![1.0; 200];
        elevations[100] = 110.0;
        elevations[150] = 110.0;
        let mut terrain_path = PTPPath::new(elevations, 100.0, 100.0, 10.0).unwrap();

        let itwom_test = ItwomPointToPoint(
            &mut terrain_path,
            PTPClimate::default(),
            5800.0,
            0.5,
            0.5,
            1,
        );

        assert_eq!(itwom_test.mode, "2_Hrzn_Diff");
        assert_eq!(itwom_test.error_num, 0);
    }
}
