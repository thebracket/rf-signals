#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/******************************************************************************

    Note that point_to_point_two has become point_to_point
    for drop-in interface to splat.cpp.
    The new variable inputs,
    double enc_ncc_clcref,
    double clutter_height,
    double clutter_density,
    double delta_h_diff, and
    int mode_var)
    have been given fixed values below.

    pol:
        0-Horizontal, 1-Vertical, 2-Circular

    radio_climate:
        1-Equatorial, 2-Continental Subtropical,
        3-Maritime Tropical, 4-Desert, 5-Continental Temperate,
        6-Maritime Temperate, Over Land, 7-Maritime Temperate,
        Over Sea

    conf, rel: .01 to .99

    elev[]: [num points - 1], [delta dist(meters)],
            [height(meters) point 1], ..., [height(meters) point n]

    clutter_height  	25.2 meters for compatibility with ITU-R P.1546-2.

    clutter_density 	1.0 for compatibility with ITU-R P.1546-2.

    delta_h_diff		optional delta h for beyond line of sight. 90 m. average.
                setting to 0.0 will default to use of original internal
                use of delta-h for beyond line-of-sight range.

    mode_var		set to 12; or to 1 for FCC ILLR;  see documentation

    enc_ncc_clcref 		clutter refractivity; 1000 N-units to match ITU-R P.1546-2

    eno=eno_ns_surfref	atmospheric refractivity at sea level; 301 N-units nominal
                (ranges from 250 for dry, hot day to 450 on hot, humid day]
                (stabilizes near 301 in cold, clear weather)

    errnum: 0- No Error.
        1- Warning: Some parameters are nearly out of range.
                    Results should be used with caution.
        2- Note: Default parameters have been substituted for
                 impossible ones.
        3- Warning: A combination of parameters is out of range.
                Results are probably invalid.
        Other-  Warning: Some parameters are out of range.
            Results are probably invalid.

*****************************************************************************/
pub fn PointToPoint(
    elev: &mut [f64],
    tht_m: f64,
    rht_m: f64,
    eps_dielect: f64,
    sgm_conductivity: f64,
    eno_ns_surfref: f64,
    frq_mhz: f64,
    radio_climate: ::std::os::raw::c_int,
    pol: ::std::os::raw::c_int,
    conf: f64,
    rel: f64,
) -> PTPResult {
    use std::ffi::CStr;
    let mut dbloss = 0.0f64;
    let mut mode = [0 as std::os::raw::c_char; 128];
    let mut errnum: std::os::raw::c_int = 0;

    unsafe {
        c::point_to_point(
            elev.as_mut_ptr(),
            tht_m,
            rht_m,
            eps_dielect,
            sgm_conductivity,
            eno_ns_surfref,
            frq_mhz,
            radio_climate,
            pol,
            conf,
            rel,
            &mut dbloss,
            mode.as_mut_ptr(),
            &mut errnum,
        );
    }

    let mode_str = unsafe { CStr::from_ptr(mode.as_ptr()).to_string_lossy() };

    PTPResult {
        dbloss: dbloss,
        mode: mode_str.to_string(),
        error_num: errnum,
    }
}

pub struct PTPResult {
    pub dbloss: f64,
    pub mode: String,
    pub error_num: i32,
}
