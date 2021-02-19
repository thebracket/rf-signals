mod units;
pub use units::{Distance, Frequency};
mod rfcalc;
pub use rfcalc::*;

//#[cfg(feature = "srtm")]
mod mapping;

//#[cfg(feature = "srtm")]
pub mod srtm {
    pub use crate::mapping::srtm::*;
}

pub mod lidar {
    pub use crate::mapping::lidar::*;
}

//#[cfg(feature = "srtm")]
pub use mapping::latlon::LatLon;
pub use mapping::{height_tile_elevations, lat_lon_tile};

// Re-export geo
//#[cfg(feature = "srtm")]
pub mod geo {
    pub use geo::*;
}

pub mod geometry;
