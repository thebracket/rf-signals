# RF_Signals

This is a Rust wrapper for [Cloud-RF/Signal Server](https://github.com/Cloud-RF/Signal-Server) algorithms. In turn, this is based upon SPLAT! by Alex Farrant and John A. Magliacane.

I've retained the GPL2 license, because the parent requires it. I needed this for work, I'm hoping someone finds it useful.

## Porting Status

|Algorithm   |Status   |
|------------|---------|
|COST/HATA   |C Only   |
|ECC33       |C Only   |
|EGLI        |C Only   |
|Fresnel Zone|Pure Rust (not in original)|
|FSPL        |Pure Rust|
|HATA        |Ported to Pure Rust|
|ITWOM3      |C Only   |
|Plane Earth |Ported to Pure Rust|
|SOIL        |Ported to Pure Rust|
|SUI         |Ported to Pure Rust|

