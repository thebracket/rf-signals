mod units;
pub use units::{Distance, Frequency};
mod itwom3;
pub use itwom3::{
    GroundConductivity, ItwomPointToPoint, PTPClimate, PTPPath, PTPResult, RadioClimate,
};
mod itwom3_port;
mod fspl;
pub use fspl::free_space_path_loss_db;
mod fresnel;
pub use fresnel::fresnel_radius;
mod cost_hata;
pub use cost_hata::cost_path_loss;
mod ecc33;
pub use ecc33::ecc33_path_loss;
mod egli;
pub use egli::egli_path_loss;
mod hata;
pub use hata::hata_path_loss;
mod pel;
pub use pel::plane_earth_path_loss;
mod soil;
pub use soil::soil_path_loss;
mod sui;
pub use sui::sui_path_loss;

pub mod ported {
    pub use super::itwom3_port::point_to_point;
}

/// Defines the calculation more for SUI, HATA, etc. path loss
#[derive(Debug, PartialEq)]
pub enum EstimateMode {
    Urban,
    Obstructed,
    Suburban,
    PartiallyObstructed,
    Rural,
    Open,
}

impl EstimateMode {
    fn to_mode(&self) -> i32 {
        match self {
            EstimateMode::Urban => 1,
            EstimateMode::Obstructed => 1,
            EstimateMode::Suburban => 2,
            EstimateMode::PartiallyObstructed => 2,
            EstimateMode::Rural => 3,
            EstimateMode::Open => 3,
        }
    }
}

// Include the C library for use in child modules.
mod c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
