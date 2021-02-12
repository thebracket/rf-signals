mod units;
pub use units::Distance;
mod itwom3;
pub use itwom3::{
    GroundConductivity, ItwomPointToPoint, PTPClimate, PTPPath, PTPResult, RadioClimate,
};
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

// Include the C library for use in child modules.
mod c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
