use super::{LidarFile, LidarHeader};
use quadtree_f32::*;
use std::path::Path;

pub struct LidarIndex {
    pub headers: Vec<LidarIndexEntry>,
    pub quadtree: Option<QuadTree>,
}

#[derive(Debug)]
pub struct LidarIndexEntry {
    pub filename: String,
    pub header: LidarHeader,
    pub data: Option<LidarFile>,
}

impl LidarIndexEntry {
    pub fn elevation(&mut self, lat: &f64, lon: &f64) -> u16 {
        if let Some(data) = &self.data {
            data.elevation(lat, lon)
        } else {
            println!("Loading {}", self.filename);
            self.data = Some(LidarFile::from_file(Path::new(&self.filename)));
            return self.data.as_ref().unwrap().elevation(lat, lon);
        }
    }

    pub fn elevation_unchecked(&self, lat: &f64, lon: &f64) -> u16 {
        if let Some(data) = &self.data {
            data.elevation(lat, lon)
        } else {
            0
        }
    }
}

#[derive(PartialEq)]
pub enum LidarCheckerResult {
    Unavailable,
    NotLoaded,
    Ready,
}

impl LidarIndex {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            quadtree: None,
        }
    }

    pub fn add_index_entry(&mut self, filename: &str, header: &LidarHeader) {
        self.headers.push(LidarIndexEntry {
            filename: filename.to_string(),
            header: *header,
            data: None,
        });
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

    pub fn is_available(&self, lat: &f64, lon: &f64) -> LidarCheckerResult {
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
            return LidarCheckerResult::Unavailable;
        }

        let mut good = true;
        id_match.iter().for_each(|id| {
            if self.headers[id.0].data.is_none() {
                good = false;
            }
        });

        if good {
            LidarCheckerResult::Ready
        } else {
            LidarCheckerResult::NotLoaded
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
            .map(|id| self.headers[id.0].elevation_unchecked(lat, lon))
            .max()
            .unwrap_or(0)
    }

    pub fn get_height_for_location_and_load(&mut self, lat: &f64, lon: &f64) -> u16 {
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
