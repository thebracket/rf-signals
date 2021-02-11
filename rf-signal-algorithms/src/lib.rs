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

// Include the C library for use in child modules.
mod c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
