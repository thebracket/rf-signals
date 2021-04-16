use crate::WISP;
use rf_signal_algorithms::{
    bheat::heat_altitude, free_space_path_loss_db, geometry::haversine_distance,
    itwom_point_to_point, lat_lon_path_1m, lat_lon_vec_to_heights, Distance, Frequency, LatLon,
    PTPClimate, PTPPath,
};

#[derive(Clone, Debug)]
pub struct WirelessService {
    tower: String,
    tower_pos: LatLon,
    name: String,
    pos: LatLon,
    height: Distance,
    frequency: Frequency,
    link_budget_db: f64,
    range: Distance,
}

#[derive(Clone, Debug)]
pub struct PossibleLink {
    pub tower: String,
    pub tower_pos: LatLon,
    pub name: String,
    pub cpe_height: f64,
    pub mode: String,
    pub signal: f64,
    pub range_km: f64,
}

/// Finds all APs within range of a given point
pub fn services_in_range(pos: &LatLon) -> Vec<WirelessService> {
    let mut result = Vec::new();

    let wisp_reader = WISP.read();
    wisp_reader.towers.iter().for_each(|t| {
        let range = haversine_distance(pos, &LatLon::new(t.lat, t.lon));

        t.access_points
            .iter()
            .filter(|ap| range.as_km() < ap.max_range_km)
            .for_each(|ap| {
                result.push(WirelessService {
                    tower: t.name.clone(),
                    tower_pos: LatLon::new(t.lat, t.lon),
                    name: ap.name.clone(),
                    pos: LatLon::new(t.lat, t.lon),
                    height: Distance::with_meters(t.height_meters),
                    frequency: Frequency::with_ghz(ap.frequency_ghz),
                    link_budget_db: ap.link_budget,
                    range: range.clone(),
                });
            });
    });

    result
}

pub fn evaluate_wireless_services(
    pos: &LatLon,
    services: &Vec<WirelessService>,
    heat_path: &str,
) -> Vec<PossibleLink> {
    let mut result = Vec::new();
    services.iter().for_each(|svc| {
        evaluate_wireless_service(pos, svc, heat_path, &mut result);
    });
    result
}

fn evaluate_wireless_service(
    pos: &LatLon,
    service: &WirelessService,
    heat_path: &str,
    results: &mut Vec<PossibleLink>,
) {
    if service.range.as_meters() < 50.0 {
        results.push(PossibleLink {
            tower: service.tower.clone(),
            tower_pos: service.tower_pos,
            name: service.name.clone(),
            cpe_height: 0.0,
            mode: "<50m Range".to_string(),
            signal: service.link_budget_db
                - free_space_path_loss_db(service.frequency, service.range),
            range_km: service.range.as_km(),
        });
    } else {
        // ITM requires that the tower not include clutter
        let base_tower_height = heat_altitude(service.pos.lat(), service.pos.lon(), heat_path)
            .unwrap_or((Distance::with_meters(0.0), Distance::with_meters(0.0)))
            .0
            .as_meters();

        // Calculate the line between tower and SM
        let path = lat_lon_path_1m(&service.pos, pos);

        // Create a list of altitudes to use
        let los_path = lat_lon_vec_to_heights(&path, heat_path);

        // Convert los_path from an array of u16 to f64 and force the tower height in spot 0.
        let mut path_as_distances: Vec<f64> = los_path.iter().map(|d| *d as f64).collect();
        path_as_distances[0] = base_tower_height;

        let mut found_los = false;
        let mut cpe_height = 0.25;
        let mut last_mode = String::new();
        while cpe_height < 3.6 && !found_los {
            let (loss, mode) = itm_eval(cpe_height, &path_as_distances, service);
            let signal = service.link_budget_db - loss;
            let mut ok = true;
            if signal < -80.0 {
                ok = false;
            }
            // Reject 5.8 or higher with 2 obstacles
            if ok && service.frequency.as_ghz() > 5.0 && mode == "2_Hrzn_Diff" {
                ok = false;
            }
            if ok && service.frequency.as_ghz() > 9.0 && mode != "L-o-S" {
                ok = false;
            }
            if mode == last_mode {
                ok = false;
            }
            if ok {
                results.push(PossibleLink {
                    tower: service.tower.clone(),
                    tower_pos: service.tower_pos,
                    name: service.name.clone(),
                    cpe_height,
                    mode: mode.clone(),
                    signal,
                    range_km: service.range.as_km(),
                });
            }
            if mode == "L-o-S" {
                found_los = true;
            } else {
                cpe_height += 0.25;
                last_mode = mode.clone();
            }
        }
    }
}

fn itm_eval(
    cpe_height: f64,
    path_as_distances: &Vec<f64>,
    service: &WirelessService,
) -> (f64, String) {
    // Setup an ITM terrain path and retrieve the data
    let mut terrain_path = PTPPath::new(
        path_as_distances.clone(),
        service.height,
        Distance::with_meters(cpe_height),
        Distance::with_meters(1.0),
    )
    .unwrap();

    let lr = itwom_point_to_point(
        &mut terrain_path,
        PTPClimate::default(),
        service.frequency,
        0.5,
        0.5,
        1,
    );

    (lr.dbloss, lr.mode)
}
