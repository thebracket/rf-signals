use crate::{Distance, Frequency};

/*
Frequency 30 to 1000MHz
h1 = 1m and above
h2 = 1m and above
Distance 1 to 50km
http://people.seas.harvard.edu/~jones/es151/prop_models/propagation.html#pel
*/
pub fn egli_path_loss(
    frequency: Frequency,
    tx_height: Distance,
    rx_height: Distance,
    distance: Distance,
) -> f64 {
    let f = frequency.as_mhz();
    let h1 = tx_height.as_meters();
    let h2 = rx_height.as_meters();
    let d = distance.as_km();

    let mut Lp50 = f64::NAN;
    let C1;
    let C2;

    if h1 > 10.0 && h2 > 10.0 {
        Lp50 = 85.9;
        C1 = 2.0;
        C2 = 2.0;
    } else if h1 > 10.0 {
        Lp50 = 76.3;
        C1 = 2.0;
        C2 = 1.0;
    } else if h2 > 10.0 {
        Lp50 = 76.3;
        C1 = 1.0;
        C2 = 2.0;
    } else
    // both antenna heights below 10 metres
    {
        Lp50 = 66.7;
        C1 = 1.0;
        C2 = 1.0;
    }

    Lp50 += 4.0 * _10log10f(d) + 2.0 * _10log10f(f) - C1 * _10log10f(h1) - C2 * _10log10f(h2);

    Lp50
}

#[inline(always)]
fn _10log10f(x: f64) -> f64 {
    4.342944 * x.ln()
}
