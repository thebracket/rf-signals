use super::{Distance, Frequency};

fn point_to_point(
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
    let mut dbloss = 0.0f64;
    let mut mode = String::new();
    let mut errnum: std::os::raw::c_int = 0;

    use super::itwom3_port::ItWomState;

    let mut itm = ItWomState::default();

    itm.point_to_point(
        elev,
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
        &mut mode,
        &mut errnum,
    );

    PTPResult {
        dbloss: dbloss,
        mode: mode,
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
    pub transmit_height: Distance,
    pub receive_height: Distance,
}

impl PTPPath {
    /// Construct a new PTP path for ITWOM evaluation. The constructor takes care of pre-pending the fields
    /// used by the C algorithm.
    pub fn new(
        elevations: Vec<f64>,
        transmit_height: Distance,
        receive_height: Distance,
        step_size: Distance,
    ) -> Result<Self, PTPError> {
        let total_distance: f64 = elevations.len() as f64 * step_size.as_meters();
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
        path.elevations.insert(0, step_size.as_meters());
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

pub fn itwom_point_to_point(
    path: &mut PTPPath,
    climate: PTPClimate,
    frequency: Frequency,
    confidence: f64,
    rel: f64,
    polarity: i32,
) -> PTPResult {
    point_to_point(
        &mut path.elevations,
        path.transmit_height.as_meters(),
        path.receive_height.as_meters(),
        climate.eps_dialect,
        climate.sgm_conductivity,
        climate.eno_ns_surfref,
        frequency.as_mhz(),
        climate.radio_climate,
        polarity,
        confidence,
        rel,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_too_short() {
        assert_eq!(
            PTPPath::new(
                vec![1.0; 2],
                Distance::with_meters(100.0),
                Distance::with_meters(100.0),
                Distance::with_meters(10.0)
            )
            .err(),
            Some(PTPError::DistanceTooShort)
        );
    }

    #[test]
    fn test_too_long() {
        assert_eq!(
            PTPPath::new(
                vec![1.0; 2000000],
                Distance::with_meters(100.0),
                Distance::with_meters(100.0),
                Distance::with_meters(10.0)
            )
            .err(),
            Some(PTPError::DistanceTooLong)
        );
    }

    #[test]
    fn altitudes_too_low() {
        assert_eq!(
            PTPPath::new(
                vec![0.4; 200],
                Distance::with_meters(100.0),
                Distance::with_meters(100.0),
                Distance::with_meters(10.0)
            )
            .err(),
            Some(PTPError::AltitudeTooLow)
        );
    }

    #[test]
    fn altitudes_too_high() {
        assert_eq!(
            PTPPath::new(
                vec![3500.0; 200],
                Distance::with_meters(100.0),
                Distance::with_meters(100.0),
                Distance::with_meters(10.0)
            )
            .err(),
            Some(PTPError::AltitudeTooHigh)
        );
    }

    #[test]
    fn basic_fspl_test() {
        let mut terrain_path = PTPPath::new(
            vec![1.0; 200],
            Distance::with_meters(100.0),
            Distance::with_meters(100.0),
            Distance::with_meters(10.0),
        )
        .unwrap();

        let itwom_test = itwom_point_to_point(
            &mut terrain_path,
            PTPClimate::default(),
            Frequency::with_mhz(5800.0),
            0.5,
            0.5,
            1,
        );

        assert_eq!(itwom_test.mode, "L-o-S");
        assert_eq!(itwom_test.error_num, 0);
        assert_eq!(itwom_test.dbloss.floor(), 113.0);
    }

    #[test]
    fn basic_one_obstruction() {
        let mut elevations = vec![1.0; 200];
        elevations[100] = 110.0;
        let mut terrain_path = PTPPath::new(
            elevations,
            Distance::with_meters(100.0),
            Distance::with_meters(100.0),
            Distance::with_meters(10.0),
        )
        .unwrap();

        let itwom_test = itwom_point_to_point(
            &mut terrain_path,
            PTPClimate::default(),
            Frequency::with_mhz(5800.0),
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
        let mut terrain_path = PTPPath::new(
            elevations,
            Distance::with_meters(100.0),
            Distance::with_meters(100.0),
            Distance::with_meters(10.0),
        )
        .unwrap();

        let itwom_test = itwom_point_to_point(
            &mut terrain_path,
            PTPClimate::default(),
            Frequency::with_mhz(5800.0),
            0.5,
            0.5,
            1,
        );

        assert_eq!(itwom_test.mode, "2_Hrzn_Diff");
        assert_eq!(itwom_test.error_num, 0);
    }
}
