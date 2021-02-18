#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod data_defs;

// Please save your API key in gmap_key.txt in the `resources` directory.
const GOOGLE_MAPS_API_KEY: &str = include_str!("../resources/gmap_key.txt");
const INDEX_HTML: &str = include_str!("../resources/index.html");
use rocket::Response;
use rocket::{http::ContentType, http::Status, response::content};

// Yes, it's a static mutable. I don't like it.
static mut INDEX_FINAL: String = String::new();

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html(unsafe { &INDEX_FINAL })
}

fn main() {
    let wisp_def = data_defs::load_wisp();

    // Do some replace magic to place the correct key and version in the HTML
    unsafe {
        INDEX_FINAL = INDEX_HTML
            .replace("_BANNER_", "Bracket-Heat 0.1")
            .replace("_GMAPKEY_", GOOGLE_MAPS_API_KEY)
            .replace("_CENTER_LAT_", &wisp_def.center.0.to_string())
            .replace("_CENTER_LON_", &wisp_def.center.1.to_string())
            .replace("_MAP_ZOOM_", &wisp_def.map_zoom.to_string())
            .replace("_ISP_NAME_", &format!("\"{}\"", &wisp_def.name));
    }
    rocket::ignite().mount("/", routes![index]).launch();
}
