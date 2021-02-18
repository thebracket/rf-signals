pub(crate) const TILE_SIZE: u32 = 256;

mod heightmap;
pub(crate) use heightmap::heightmap_tile;
mod losmap;
pub(crate) use losmap::losmap_tile;
