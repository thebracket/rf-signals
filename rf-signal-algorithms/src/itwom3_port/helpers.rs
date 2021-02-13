// The idea here is to deprecate these and port them to proper Rust
// syntax.

pub(crate) fn pow(n: f64, e: f64) -> f64 {
    n.powf(e)
}

pub(crate) fn abs(n: f64) -> f64 {
    n.abs()
}

pub(crate) fn fabs(n: f64) -> f64 {
    n.abs()
}

pub(crate) fn log(n: f64) -> f64 {
    n.ln()
}

pub(crate) fn log10(n: f64) -> f64 {
    n.log10()
}

pub(crate) fn cos(n: f64) -> f64 {
    n.cos()
}

pub(crate) fn sin(n: f64) -> f64 {
    n.sin()
}

pub(crate) fn acos(n: f64) -> f64 {
    n.acos()
}

pub(crate) fn asin(n: f64) -> f64 {
    n.asin()
}

pub(crate) fn exp(n: f64) -> f64 {
    n.exp()
}

pub(crate) fn sqrt(n: f64) -> f64 {
    n.sqrt()
}

pub(crate) fn mymax(a: f64, b: f64) -> f64
{
	if a > b {
		return a;
    }
	else {
		return b;
    }
}

pub(crate) fn FORTRAN_DIM(x: f64, y: f64) -> f64 {
	/* This performs the FORTRAN DIM function.  Result is x-y
	   if x is greater than y; otherwise result is 0.0 */

	if x > y {
		return x - y;
    } else {
		return 0.0;
    }
}