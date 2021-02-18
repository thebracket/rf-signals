use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;
use super::Tower;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Wisp {
    pub name: String,
    pub center: (f64, f64),
    pub map_zoom: u32,
    pub towers: Vec<Tower>
}

pub fn load_wisp() -> Wisp {
    let f = File::open("resources/isp.ron").unwrap();
    let wisp: Wisp = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            panic!("Unable to load WISP definition file. Is it in resources?");
        }
    };
    wisp
}
