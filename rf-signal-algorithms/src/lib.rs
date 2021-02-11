mod itwom3;
pub use itwom3::{
    GroundConductivity, ItwomPointToPoint, PTPClimate, PTPPath, PTPResult, RadioClimate,
};

// Include the C library for use in child modules.
mod c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
