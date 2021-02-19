use super::LidarHeader;
use memmap::Mmap;
use quadtree_f32::*;

pub struct LidarIndex {
    pub headers: Vec<LidarIndexEntry>,
    pub quadtree: Option<QuadTree>,
}

#[derive(Debug)]
pub struct LidarIndexEntry {
    pub header: LidarHeader,
    pub memory: Mmap,
}

impl LidarIndexEntry {
    pub fn elevation(&self, point_lat: &f64, point_lon: &f64) -> u16 {
        const SIZE_OF_HEADER: usize = std::mem::size_of::<LidarHeader>();

        let lat_span = self.header.max_lat - self.header.min_lat;
        let lon_span = self.header.max_lon - self.header.min_lon;
        let lat = (point_lat - self.header.min_lat) / lat_span;
        let lon = (point_lon - self.header.min_lon) / lon_span;
        let row_idx = (lon * self.header.size as f64) as usize;
        let col_idx = (lat * self.header.size as f64) as usize;

        let base_idx = (row_idx * self.header.size) + col_idx;

        let memsize = self.memory.len();
        bytemuck::cast_slice(&self.memory[SIZE_OF_HEADER..memsize])[base_idx]
    }
}

impl LidarIndex {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            quadtree: None,
        }
    }

    pub fn add_index_entry(&mut self, header: LidarHeader, memory: Mmap) {
        self.headers.push(LidarIndexEntry { header, memory });
    }

    pub fn bake_quadtree(&mut self) {
        let items = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, index)| {
                (
                    ItemId(i),
                    Item::Rect(Rect {
                        max_y: index.header.max_lon as f32,
                        min_y: index.header.min_lon as f32,
                        max_x: index.header.max_lat as f32,
                        min_x: index.header.min_lat as f32,
                    }),
                )
            })
            .collect::<Vec<_>>();
        self.quadtree = Some(QuadTree::new(items.into_iter()));
    }

    pub fn is_available(&self, lat: &f64, lon: &f64) -> bool {
        let target = Rect {
            min_y: *lon as f32,
            max_y: *lon as f32,
            min_x: *lat as f32,
            max_x: *lat as f32,
        };
        let id_match = self
            .quadtree
            .as_ref()
            .unwrap()
            .get_ids_that_overlap(&target);

        // Quick return if there's no data
        if id_match.is_empty() {
            false
        } else {
            true
        }
    }

    pub fn get_height_for_location(&self, lat: &f64, lon: &f64) -> u16 {
        let target = Rect {
            min_y: *lon as f32,
            max_y: *lon as f32,
            min_x: *lat as f32,
            max_x: *lat as f32,
        };
        let id_match = self
            .quadtree
            .as_ref()
            .unwrap()
            .get_ids_that_overlap(&target);

        // Quick return if there's no data
        if id_match.is_empty() {
            return 0;
        }

        id_match
            .iter()
            .map(|id| self.headers[id.0].elevation(lat, lon))
            .max()
            .unwrap_or(0)
    }
}
