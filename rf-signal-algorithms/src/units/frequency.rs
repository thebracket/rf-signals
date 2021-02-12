/// Type to represent a radio frequency in Hz
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Frequency(pub f64);

impl Frequency {
    /// Specify frequency as Hz
    pub fn with_hz<T: Into<f64>>(hz: T) -> Self {
        Self(hz.into())
    }

    /// Specify frequency as Mhz
    pub fn with_mhz<T: Into<f64>>(mhz: T) -> Self {
        Self(mhz.into() * 1_000.0)
    }

    /// Specify frequency as Ghz
    pub fn with_ghz<T: Into<f64>>(ghz: T) -> Self {
        Self(ghz.into() * 1_000_000.0)
    }

    /// Retrieve frequency as Hz
    pub fn as_hz(&self) -> f64 {
        self.0
    }

    /// Retrieve frequency as Mhz
    pub fn as_mhz(&self) -> f64 {
        self.0 / 1_000.0
    }

    /// Retrieve frequency as Ghz
    pub fn as_ghz(&self) -> f64 {
        self.0 / 1_000_000.0
    }
}
