use memmap::{Mmap, MmapOptions};
use std::io::prelude::*;
use std::path::Path;
use std::{fs::File, io::BufReader, mem::size_of};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LidarHeader {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
    pub size: usize,
}

unsafe impl bytemuck::Zeroable for LidarHeader {}
unsafe impl bytemuck::Pod for LidarHeader {}

#[derive(Debug, Clone)]
pub struct LidarFile {
    pub header: LidarHeader,
    pub data: Vec<u16>,
}

impl LidarFile {
    pub fn new(header: LidarHeader) -> Self {
        let size = header.size;
        Self {
            header,
            data: vec![0u16; size * (size + 1)],
        }
    }

    pub fn just_header(path: &Path) -> LidarHeader {
        const SIZE_OF_HEADER: usize = size_of::<LidarHeader>();
        let f = File::open(path).unwrap();
        let mapped_file = unsafe { MmapOptions::new().map(&f).unwrap() };
        bytemuck::from_bytes::<LidarHeader>(&mapped_file[0..SIZE_OF_HEADER]).clone()
    }

    pub fn header_and_mmap(path: &Path) -> (LidarHeader, Mmap) {
        const SIZE_OF_HEADER: usize = size_of::<LidarHeader>();
        let f = File::open(path).unwrap();
        let mapped_file = unsafe { MmapOptions::new().map(&f).unwrap() };
        (
            bytemuck::from_bytes::<LidarHeader>(&mapped_file[0..SIZE_OF_HEADER]).clone(),
            mapped_file,
        )
    }

    pub fn from_file(path: &Path) -> Self {
        const SIZE_OF_HEADER: usize = size_of::<LidarHeader>();
        let mut file = BufReader::new(File::open(path).expect("Cannot open file"));
        let mut hdr = [0u8; SIZE_OF_HEADER];
        let bytes_read = file.read(&mut hdr).unwrap();
        assert_eq!(bytes_read, SIZE_OF_HEADER);

        let header = bytemuck::from_bytes::<LidarHeader>(&hdr);
        let size = header.size;

        let mut lf = LidarFile {
            header: header.clone(),
            data: Vec::with_capacity(size * (size + 1)),
        };

        let size_of_data = size * (size + 1);
        let mut h = [0u8; 2];
        for _ in 0..size_of_data {
            file.read(&mut h).unwrap();
            let h = bytemuck::from_bytes::<u16>(&h);
            lf.data.push(*h);
        }

        lf
    }

    pub fn set_point(&mut self, point_lat: f64, point_lon: f64, altitude: u16) {
        let lat_span = self.header.max_lat - self.header.min_lat;
        let lon_span = self.header.max_lon - self.header.min_lon;
        let lat = (point_lat - self.header.min_lat) / lat_span;
        let lon = (point_lon - self.header.min_lon) / lon_span;
        let row_idx = (lon * self.header.size as f64) as usize;
        let col_idx = (lat * self.header.size as f64) as usize;
        let idx = (row_idx * self.header.size) + col_idx;
        if self.data[idx] < altitude {
            self.data[idx] = altitude;
        }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(bytemuck::cast_slice(&[self.header]))?;
        file.write_all(bytemuck::cast_slice(&self.data))?;
        Ok(())
    }

    pub fn in_bounds(&self, lat: f64, lon: f64) -> bool {
        lat >= self.header.min_lat
            && lat < self.header.max_lat
            && lon >= self.header.min_lon
            && lon < self.header.max_lon
    }

    pub fn elevation(&self, point_lat: &f64, point_lon: &f64) -> u16 {
        let lat_span = self.header.max_lat - self.header.min_lat;
        let lon_span = self.header.max_lon - self.header.min_lon;
        let lat = (point_lat - self.header.min_lat) / lat_span;
        let lon = (point_lon - self.header.min_lon) / lon_span;
        let row_idx = (lon * self.header.size as f64) as usize;
        let col_idx = (lat * self.header.size as f64) as usize;
        let idx = usize::min((row_idx * self.header.size) + col_idx, self.data.len() - 1);
        self.data[idx]
    }
}
