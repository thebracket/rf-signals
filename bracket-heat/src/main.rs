#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use lazy_static::*;
use parking_lot::RwLock;
mod data_defs;
use data_defs::*;
mod tiler;

// Please save your API key in gmap_key.txt in the `resources` directory.
const GOOGLE_MAPS_API_KEY: &str = include_str!("../resources/gmap_key.txt");
const INDEX_HTML: &str = include_str!("../resources/index.html");
use rocket::Response;
use rocket::{http::ContentType, http::Status, response::content};
use rocket_contrib::json::Json;

// Data Storage Holders
lazy_static! {
    static ref INDEX_FINAL: RwLock<String> = RwLock::new(String::new());
}

lazy_static! {
    static ref WISP: RwLock<Wisp> = RwLock::new(Wisp::default());
}

#[get("/")]
fn index() -> content::Html<String> {
    content::Html(INDEX_FINAL.read().clone())
}

#[get("/tower_Marker.png")]
fn tower_marker<'a>() -> rocket::response::Stream<std::fs::File> {
    use std::fs::File;
    rocket::response::Stream::from(File::open("resources/tower_Marker.png").unwrap())
}

#[get("/towers", format = "json")]
fn towers() -> Json<Vec<Tower>> {
    Json(WISP.read().towers.clone())
}

#[get("/heightmap/<swlat>/<swlon>/<nelat>/<nelon>")]
fn heightmap<'a>(swlat: f64, swlon: f64, nelat: f64, nelon: f64) -> Response<'a> {
    let image_buffer = tiler::heightmap_tile(swlat, swlon, nelat, nelon);
    let mut response_build = Response::build();
    response_build.header(ContentType::PNG);
    response_build.status(Status::Ok);
    response_build.streamed_body(std::io::Cursor::new(image_buffer));
    response_build.finalize()
}

#[get("/losmap/<swlat>/<swlon>/<nelat>/<nelon>")]
fn losmap<'a>(swlat: f64, swlon: f64, nelat: f64, nelon: f64) -> Response<'a> {
    let image_buffer = tiler::losmap_tile(swlat, swlon, nelat, nelon);
    let mut response_build = Response::build();
    response_build.header(ContentType::PNG);
    response_build.status(Status::Ok);
    response_build.streamed_body(std::io::Cursor::new(image_buffer));
    response_build.finalize()
}

fn main() {
    let wisp_def = load_wisp();

    // Do some replace magic to place the correct key and version in the HTML
    *INDEX_FINAL.write() = INDEX_HTML
        .replace("_BANNER_", "Bracket-Heat 0.1")
        .replace("_GMAPKEY_", GOOGLE_MAPS_API_KEY)
        .replace("_CENTER_LAT_", &wisp_def.center.0.to_string())
        .replace("_CENTER_LON_", &wisp_def.center.1.to_string())
        .replace("_MAP_ZOOM_", &wisp_def.map_zoom.to_string())
        .replace("_ISP_NAME_", &format!("\"{}\"", &wisp_def.name));

    *WISP.write() = wisp_def;

    println!("Indexing LiDAR Data - Please Wait");
    rf_signal_algorithms::lidar::index_all_lidar("z:/lidarserver/terrain/lidar");

    rocket::ignite()
        .mount("/", routes![index, tower_marker, towers, heightmap, losmap])
        .launch();
}