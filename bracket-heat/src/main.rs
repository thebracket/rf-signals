#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use lazy_static::*;
use parking_lot::RwLock;
mod data_defs;
use data_defs::*;
mod calculators;
mod los;
mod tiler;

// Please save your API key in gmap_key.txt in the `resources` directory.
const GOOGLE_MAPS_API_KEY: &str = include_str!("../resources/gmap_key.txt");
const INDEX_HTML: &str = include_str!("../resources/index.html");
const ADVISOR_HTML: &str = include_str!("../resources/advisor.html");
use rf_signal_algorithms::{Frequency, LatLon};
use rocket::{config::Environment, Config, Response};
use rocket::{http::ContentType, http::Status, response::content};
use rocket_contrib::json::Json;

// Data Storage Holders
lazy_static! {
    static ref INDEX_FINAL: RwLock<String> = RwLock::new(String::new());
}

lazy_static! {
    static ref ADVISOR_FINAL: RwLock<String> = RwLock::new(String::new());
}

lazy_static! {
    static ref WISP: RwLock<Wisp> = RwLock::new(Wisp::default());
}

#[get("/")]
fn index() -> content::Html<String> {
    content::Html(INDEX_FINAL.read().clone())
}

#[get("/advisor.html")]
fn advisor() -> content::Html<String> {
    content::Html(ADVISOR_FINAL.read().clone())
}

#[get("/three.js")]
fn three_js<'a>() -> rocket::response::Stream<std::fs::File> {
    use std::fs::File;
    rocket::response::Stream::from(File::open("resources/three.js").unwrap())
}

#[get("/locinfo.html")]
fn loc_info<'a>() -> rocket::response::Stream<std::fs::File> {
    use std::fs::File;
    rocket::response::Stream::from(File::open("resources/locinfo.html").unwrap())
}

#[get("/tower_Marker.png")]
fn tower_marker<'a>() -> rocket::response::Stream<std::fs::File> {
    use std::fs::File;
    rocket::response::Stream::from(File::open("resources/tower_Marker.png").unwrap())
}

#[get("/pngegg.png")]
fn pngegg<'a>() -> rocket::response::Stream<std::fs::File> {
    use std::fs::File;
    rocket::response::Stream::from(File::open("resources/pngegg.png").unwrap())
}

#[get("/towers", format = "json")]
fn towers() -> Json<Vec<Tower>> {
    Json(WISP.read().towers.clone())
}

#[get("/budgets", format = "json")]
fn budgets() -> Json<Vec<LinkBudget>> {
    Json(WISP.read().link_budgets.clone())
}

#[get("/heightmap/<swlat>/<swlon>/<nelat>/<nelon>")]
fn heightmap<'a>(swlat: f64, swlon: f64, nelat: f64, nelon: f64) -> Response<'a> {
    let heat_path = WISP.read().heat_path.clone();
    let image_buffer = tiler::heightmap_tile(swlat, swlon, nelat, nelon, &heat_path);
    let mut response_build = Response::build();
    response_build.header(ContentType::PNG);
    response_build.status(Status::Ok);
    response_build.streamed_body(std::io::Cursor::new(image_buffer));
    response_build.finalize()
}

#[get("/heightmap_detail/<swlat>/<swlon>/<nelat>/<nelon>")]
fn heightmap_detail<'a>(swlat: f64, swlon: f64, nelat: f64, nelon: f64) -> Response<'a> {
    let heat_path = WISP.read().heat_path.clone();
    let image_buffer = tiler::heightmap_detail(swlat, swlon, nelat, nelon, &heat_path);
    let mut response_build = Response::build();
    response_build.header(ContentType::PNG);
    response_build.status(Status::Ok);
    response_build.streamed_body(std::io::Cursor::new(image_buffer));
    response_build.finalize()
}

#[get("/losmap/<swlat>/<swlon>/<nelat>/<nelon>/<cpe_height>")]
fn losmap<'a>(swlat: f64, swlon: f64, nelat: f64, nelon: f64, cpe_height: f64) -> Response<'a> {
    let heat_path = WISP.read().heat_path.clone();
    let image_buffer = tiler::losmap_tile(swlat, swlon, nelat, nelon, cpe_height, &heat_path);
    let mut response_build = Response::build();
    response_build.header(ContentType::PNG);
    response_build.status(Status::Ok);
    response_build.streamed_body(std::io::Cursor::new(image_buffer));
    response_build.finalize()
}

#[get("/signalmap/<swlat>/<swlon>/<nelat>/<nelon>/<cpe_height>/<frequency>/<link_budget>")]
fn signalmap<'a>(
    swlat: f64,
    swlon: f64,
    nelat: f64,
    nelon: f64,
    cpe_height: f64,
    frequency: f64,
    link_budget: f64,
) -> Response<'a> {
    let heat_path = WISP.read().heat_path.clone();
    let image_buffer = tiler::signalmap_tile(
        swlat,
        swlon,
        nelat,
        nelon,
        cpe_height,
        frequency,
        &heat_path,
        link_budget,
    );
    let mut response_build = Response::build();
    response_build.header(ContentType::PNG);
    response_build.status(Status::Ok);
    response_build.streamed_body(std::io::Cursor::new(image_buffer));
    response_build.finalize()
}

#[get("/signalmap_detail/<swlat>/<swlon>/<nelat>/<nelon>")]
fn signalmap_detail<'a>(swlat: f64, swlon: f64, nelat: f64, nelon: f64) -> Response<'a> {
    let heat_path = WISP.read().heat_path.clone();
    let image_buffer = tiler::signalmap_detail(swlat, swlon, nelat, nelon, &heat_path);
    let mut response_build = Response::build();
    response_build.header(ContentType::PNG);
    response_build.status(Status::Ok);
    response_build.streamed_body(std::io::Cursor::new(image_buffer));
    response_build.finalize()
}

#[get(
    "/mapclick/<lat>/<lon>/<cpe_height>/<frequency>/<link_budget>",
    format = "json"
)]
fn map_click<'a>(
    lat: f64,
    lon: f64,
    frequency: f64,
    cpe_height: f64,
    link_budget: f64,
) -> Json<los::ClickSite> {
    let heat_path = WISP.read().heat_path.clone();
    let pos = LatLon::new(lat, lon);
    Json(los::evaluate_tower_click(
        &pos,
        Frequency::with_ghz(frequency),
        cpe_height,
        &heat_path,
        link_budget,
    ))
}

#[get("/3d/<lat>/<lon>", format = "json")]
fn tile3d(lat: f64, lon: f64) -> Json<tiler::TerrainBlob> {
    let heat_path = WISP.read().heat_path.clone();
    Json(tiler::build_3d_heightmap(lat, lon, &heat_path))
}

#[get("/losplot/<lat>/<lon>/<tower_name>/<cpe_height>/<frequency>")]
fn los_plot<'a>(
    lat: f64,
    lon: f64,
    tower_name: String,
    cpe_height: f64,
    frequency: f64,
) -> Json<los::LineOfSightPlot> {
    let tower_index = WISP
        .read()
        .towers
        .iter()
        .enumerate()
        .find(|(_i, t)| t.name == tower_name)
        .map(|(i, _)| i)
        .unwrap();
    let heat_path = WISP.read().heat_path.clone();
    let pos = LatLon::new(lat, lon);
    Json(los::los_plot(
        &pos,
        tower_index,
        cpe_height,
        Frequency::with_ghz(frequency),
        &heat_path,
    ))
}

fn main() {
    let wisp_def = load_wisp();

    // Do some replace magic to place the correct key and version in the HTML
    *INDEX_FINAL.write() = INDEX_HTML
        .replace("_BANNER_", "Bracket-Heat 0.1")
        .replace("_GMAPKEY_", &GOOGLE_MAPS_API_KEY.replace("\n", ""))
        .replace("_CENTER_LAT_", &wisp_def.center.0.to_string())
        .replace("_CENTER_LON_", &wisp_def.center.1.to_string())
        .replace("_MAP_ZOOM_", &wisp_def.map_zoom.to_string())
        .replace("_ISP_NAME_", &format!("\"{}\"", &wisp_def.name));

    *ADVISOR_FINAL.write() = ADVISOR_HTML
        .replace("_BANNER_", "Bracket-Heat 0.1")
        .replace("_GMAPKEY_", &GOOGLE_MAPS_API_KEY.replace("\n", ""))
        .replace("_CENTER_LAT_", &wisp_def.center.0.to_string())
        .replace("_CENTER_LON_", &wisp_def.center.1.to_string())
        .replace("_MAP_ZOOM_", &wisp_def.map_zoom.to_string())
        .replace("_ISP_NAME_", &format!("\"{}\"", &wisp_def.name));

    let config = Config::build(Environment::Production)
        .port(wisp_def.listen_port)
        .finalize()
        .unwrap();

    *WISP.write() = wisp_def;

    rocket::custom(config)
        .mount(
            "/",
            routes![
                index,
                advisor,
                tower_marker,
                towers,
                heightmap,
                heightmap_detail,
                losmap,
                signalmap,
                signalmap_detail,
                map_click,
                pngegg,
                los_plot,
                budgets,
                three_js,
                loc_info,
                tile3d,
            ],
        )
        .launch();
}
