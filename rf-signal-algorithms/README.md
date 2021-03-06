# RF_Signals

This is a Rust wrapper for [Cloud-RF/Signal Server](https://github.com/Cloud-RF/Signal-Server) algorithms. In turn, this is based upon SPLAT! by Alex Farrant and John A. Magliacane.

I've retained the GPL2 license, because the parent requires it. I needed this for work, I'm hoping someone finds it useful.

## Support Algorithms

This crate provides Rust implementations of a number of algorithms that are useful in wireless calculations:

* ITM3/Longley-Rice - the power behind Splat! and other functions.
* HATA with the COST123 extension.
* ECC33.
* EGLI.
* HATA.
* Plane Earth.
* SOIL.
* SUI.

Additionally, helper functions provide:

* Basic Free-Space Path Loss (FSPL) calculation.
* Fresnel size calculation.

## SRTM .hgt Reader

There's also an SRTM .hgt reader. You can get these from various places for pretty much the whole planet. See Radio Mobile for details. This will eventually be in its own feature. For now, it maintains an LRU cache of height tiles and tries to find the best resolution available to answer an elevation query.

An example query:

```rust
let loc = LatLon::new(38.947775, -92.323385);
let altitude = get_altitude(&loc, "resources");
```

This requires the `hgt` files from the `resources` directory to function.

## Porting Status

All algorithms started out in Cloud_RF's Signal Server (in C or C++) and were ported to Rust.

|Algorithm   |Status   |
|------------|---------|
|COST/HATA   |Ported to Pure Rust|
|ECC33       |Ported to Pure Rust|
|EGLI        |Ported to Pure Rust|
|Fresnel Zone|Pure Rust (not in original)|
|FSPL        |Pure Rust|
|HATA        |Ported to Pure Rust|
|ITWOM3      |Ported to Pure Rust|
|Plane Earth |Ported to Pure Rust|
|SOIL        |Ported to Pure Rust|
|SUI         |Ported to Pure Rust|

