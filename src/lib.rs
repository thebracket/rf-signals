mod itwom3;
pub use itwom3::{PTPPath, PTPClimate, GroundConductivity, RadioClimate, PTPResult, ItwomPointToPoint};

// Include the C library for use in child modules.
mod c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
