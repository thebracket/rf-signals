/// Provides distance unit conversions for this crate.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Distance(pub f64);

impl Distance {
    /// Specify a distance in meters
    pub fn with_meters<T: Into<f64>>(meters: T) -> Self {
        Self(meters.into())
    }

    /// Specify a distance in km
    pub fn with_kilometers<T: Into<f64>>(km: T) -> Self {
        Self(km.into() * 1000.0)
    }

    /// Specify a distance in feet
    pub fn with_feet<T: Into<f64>>(feet: T) -> Self {
        Self(feet.into() * 0.3048)
    }

    /// Specify a distance in miles
    pub fn with_miles<T: Into<f64>>(miles: T) -> Self {
        Self(miles.into() * 1609.34)
    }

    /// Retrieve the distance as meters
    pub fn as_meters(&self) -> f64 {
        self.0
    }

    /// Retrieve the distance as kilometers
    pub fn as_km(&self) -> f64 {
        self.0 / 1000.0
    }
}

#[cfg(test)]
mod test {
    use super::Distance;

    #[test]
    pub fn meters_from_f32() {
        let m = Distance::with_meters(10.0f32);
        assert_eq!(m.0, 10.0f64);
    }

    #[test]
    pub fn meters_from_f64() {
        let m = Distance::with_meters(10.0f64);
        assert_eq!(m.0, 10.0f64);
    }

    #[test]
    pub fn meters_from_km() {
        let m = Distance::with_kilometers(1.0);
        assert_eq!(1000.0, m.0);
    }

    #[test]
    pub fn meters_from_feet() {
        let m = Distance::with_feet(1.0);
        assert_eq!(0.3048, m.0);
    }

    #[test]
    pub fn meters_from_miles() {
        let m = Distance::with_miles(1.0);
        assert_eq!(1609.34, m.0);
    }

    #[test]
    pub fn mile_to_km() {
        let m = Distance::with_miles(1.0);
        let km = m.as_km();
        assert_eq!(1.60934, km);
    }
}
