mod data_defs;
use data_defs::*;
mod calculators;
mod los;
mod tiler;

use rf_signal_algorithms::{Frequency, LatLon};
use rocket::{fs::NamedFile, get, http::ContentType, response::content, routes, serde::json::Json};
use std::sync::{LazyLock, RwLock};

// Please save your API key in gmap_key.txt in the `resources` directory.
const GOOGLE_MAPS_API_KEY: &str = include_str!("../resources/gmap_key.txt");
const INDEX_HTML: &str = include_str!("../resources/index.html");
const ADVISOR_HTML: &str = include_str!("../resources/advisor.html");

// Data Storage Holders
static INDEX_FINAL: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new(String::new()));
static ADVISOR_FINAL: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new(String::new()));
static WISP: LazyLock<RwLock<Wisp>> = LazyLock::new(|| RwLock::new(Wisp::default()));

#[get("/")]
fn index() -> content::RawHtml<String> {
    content::RawHtml(INDEX_FINAL.read().unwrap().clone())
}

#[get("/advisor.html")]
fn advisor() -> content::RawHtml<String> {
    content::RawHtml(ADVISOR_FINAL.read().unwrap().clone())
}

#[get("/locinfo.html")]
async fn loc_info() -> Option<NamedFile> {
    NamedFile::open("resources/locinfo.html").await.ok()
}

#[get("/tower_Marker.png")]
async fn tower_marker() -> Option<NamedFile> {
    NamedFile::open("resources/tower_Marker.png").await.ok()
}

#[get("/pngegg.png")]
async fn pngegg() -> Option<NamedFile> {
    NamedFile::open("resources/pngegg.png").await.ok()
}

#[get("/towers", format = "json")]
fn towers() -> Json<Vec<Tower>> {
    Json(WISP.read().unwrap().towers.clone())
}

#[get("/budgets", format = "json")]
fn budgets() -> Json<Vec<LinkBudget>> {
    Json(WISP.read().unwrap().link_budgets.clone())
}

fn png_response(image_buffer: Vec<u8>) -> (ContentType, Vec<u8>) {
    (ContentType::PNG, image_buffer)
}

#[get("/heightmap/<swlat>/<swlon>/<nelat>/<nelon>")]
fn heightmap(swlat: f64, swlon: f64, nelat: f64, nelon: f64) -> (ContentType, Vec<u8>) {
    let heat_path = WISP.read().unwrap().heat_path.clone();
    png_response(tiler::heightmap_tile(
        swlat, swlon, nelat, nelon, &heat_path,
    ))
}

#[get("/heightmap_detail/<swlat>/<swlon>/<nelat>/<nelon>")]
fn heightmap_detail(swlat: f64, swlon: f64, nelat: f64, nelon: f64) -> (ContentType, Vec<u8>) {
    let heat_path = WISP.read().unwrap().heat_path.clone();
    png_response(tiler::heightmap_detail(
        swlat, swlon, nelat, nelon, &heat_path,
    ))
}

#[get("/losmap/<swlat>/<swlon>/<nelat>/<nelon>/<cpe_height>")]
fn losmap(
    swlat: f64,
    swlon: f64,
    nelat: f64,
    nelon: f64,
    cpe_height: f64,
) -> (ContentType, Vec<u8>) {
    let heat_path = WISP.read().unwrap().heat_path.clone();
    png_response(tiler::losmap_tile(
        swlat, swlon, nelat, nelon, cpe_height, &heat_path,
    ))
}

#[get("/signalmap/<swlat>/<swlon>/<nelat>/<nelon>/<cpe_height>/<frequency>/<link_budget>")]
fn signalmap(
    swlat: f64,
    swlon: f64,
    nelat: f64,
    nelon: f64,
    cpe_height: f64,
    frequency: f64,
    link_budget: f64,
) -> (ContentType, Vec<u8>) {
    let heat_path = WISP.read().unwrap().heat_path.clone();
    png_response(tiler::signalmap_tile(
        swlat,
        swlon,
        nelat,
        nelon,
        cpe_height,
        frequency,
        &heat_path,
        link_budget,
    ))
}

#[get("/signalmap_detail/<swlat>/<swlon>/<nelat>/<nelon>")]
fn signalmap_detail(swlat: f64, swlon: f64, nelat: f64, nelon: f64) -> (ContentType, Vec<u8>) {
    let heat_path = WISP.read().unwrap().heat_path.clone();
    png_response(tiler::signalmap_detail(
        swlat, swlon, nelat, nelon, &heat_path,
    ))
}

#[get(
    "/mapclick/<lat>/<lon>/<cpe_height>/<frequency>/<link_budget>",
    format = "json"
)]
fn map_click(
    lat: f64,
    lon: f64,
    frequency: f64,
    cpe_height: f64,
    link_budget: f64,
) -> Json<los::ClickSite> {
    let heat_path = WISP.read().unwrap().heat_path.clone();
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
    let heat_path = WISP.read().unwrap().heat_path.clone();
    Json(tiler::build_3d_heightmap(lat, lon, &heat_path))
}

#[get("/losplot/<lat>/<lon>/<tower_name>/<cpe_height>/<frequency>")]
fn los_plot(
    lat: f64,
    lon: f64,
    tower_name: String,
    cpe_height: f64,
    frequency: f64,
) -> Json<los::LineOfSightPlot> {
    let tower_index = WISP
        .read()
        .unwrap()
        .towers
        .iter()
        .enumerate()
        .find(|(_i, t)| t.name == tower_name)
        .map(|(i, _)| i)
        .unwrap();
    let heat_path = WISP.read().unwrap().heat_path.clone();
    let pos = LatLon::new(lat, lon);
    Json(los::los_plot(
        &pos,
        tower_index,
        cpe_height,
        Frequency::with_ghz(frequency),
        &heat_path,
    ))
}

#[rocket::main]
async fn main() {
    let wisp_def = load_wisp();
    let listen_port = wisp_def.listen_port;

    // Do some replace magic to place the correct key and version in the HTML
    *INDEX_FINAL.write().unwrap() = INDEX_HTML
        .replace("_BANNER_", "Bracket-Heat 0.1")
        .replace("_GMAPKEY_", &GOOGLE_MAPS_API_KEY.replace("\n", ""))
        .replace("_CENTER_LAT_", &wisp_def.center.0.to_string())
        .replace("_CENTER_LON_", &wisp_def.center.1.to_string())
        .replace("_MAP_ZOOM_", &wisp_def.map_zoom.to_string())
        .replace("_ISP_NAME_", &format!("\"{}\"", &wisp_def.name));

    *ADVISOR_FINAL.write().unwrap() = ADVISOR_HTML
        .replace("_BANNER_", "Bracket-Heat 0.1")
        .replace("_GMAPKEY_", &GOOGLE_MAPS_API_KEY.replace("\n", ""))
        .replace("_CENTER_LAT_", &wisp_def.center.0.to_string())
        .replace("_CENTER_LON_", &wisp_def.center.1.to_string())
        .replace("_MAP_ZOOM_", &wisp_def.map_zoom.to_string())
        .replace("_ISP_NAME_", &format!("\"{}\"", &wisp_def.name));

    *WISP.write().unwrap() = wisp_def;

    let config = rocket::Config::figment().merge(("port", listen_port));
    if let Err(e) = rocket::custom(config)
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
                loc_info,
                tile3d,
            ],
        )
        .launch()
        .await
    {
        eprintln!("Rocket failed to launch: {e}");
    }
}
