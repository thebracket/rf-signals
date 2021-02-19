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
pub use mapping::{height_tile_elevations, lat_lon_tile, lat_lon_path_10m, lat_lon_vec_to_heights, has_line_of_sight};

// Re-export geo
//#[cfg(feature = "srtm")]
pub mod geo {
    pub use geo::*;
}

pub mod geometry;
