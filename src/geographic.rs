//! Geographic system of coordinates.

use num_traits::{Euclid, Float, FloatConst, Signed};

use crate::{cartesian::Cartesian, positive::Positive};

/// Represents the horizontal axis in a geographic system of coordinates.
///
/// ## Definition
/// Since the longitude of a point on a sphere is the angle east (positive) or west (negative) in
/// reference of the maridian zero, the longitude value must be in the range __[-π, +π)__.
/// Any other value will be computed in order to set its equivalent inside that range.
///
/// ### Overflow
/// Both boundaries of the longitude range are consecutive, which means that overflowing one is the
/// same as continuing from the other one in the same direction.
///
/// ## Example
/// ```
/// use std::f64::consts::PI;
///
/// use geocart::{geographic::Longitude};
///
/// assert_eq!(
///     Longitude::from(PI + 1.),
///     Longitude::from(-PI + 1.)
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Longitude<T>(T);

impl<T> From<T> for Longitude<T>
where
    T: Copy + PartialOrd + Signed + FloatConst + Euclid,
{
    fn from(value: T) -> Self {
        Self(if (-T::PI()..T::PI()).contains(&value) {
            value
        } else {
            // Both boundaries of the range are consecutive, which means that
            // overflowing one is the same as continuing from the other one
            // in the same direction.
            (value + T::PI()).rem_euclid(&T::TAU()) - T::PI()
        })
    }
}

impl<T> From<Cartesian<T>> for Longitude<T>
where
    T: PartialOrd + Signed + Float + FloatConst + Euclid,
{
    /// Computes the [Longitude] of the given [Cartesian] as specified by the [Spherical coordinate
    /// system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(point: Cartesian<T>) -> Self {
        match (point.x, point.y) {
            (x, y) if x > T::zero() => (y / x).atan(),
            (x, y) if x < T::zero() && y >= T::zero() => (y / x).atan() + T::PI(),
            (x, y) if x < T::zero() && y < T::zero() => (y / x).atan() - T::PI(),
            (x, y) if x == T::zero() && y > T::zero() => T::FRAC_PI_2(),
            (x, y) if x == T::zero() && y < T::zero() => -T::FRAC_PI_2(),
            (x, y) if x == T::zero() && y == T::zero() => T::zero(), // fallback value

            _ => T::zero(), // fallback value
        }
        .into()
    }
}

impl<T> Longitude<T> {
    /// Returns the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

/// Represents the vertical axis in a geographic system of coordinates.
///
/// ## Definition
/// Since the latitude of a point on a sphere is the angle between the equatorial plane and the
/// straight line that goes through that point and the center of the sphere, the latitude value
/// must be in the range __\[-π/2, +π/2\]__.
/// Any other value must be computed in order to set its equivalent inside the range.
///
/// ### Overflow
/// Overflowing any of both boundaries of the latitude range behaves like moving away from that
/// boundary and getting closer to the oposite one.
///
/// ## Example
/// ```
/// use std::f64::consts::PI;
///
/// use geocart::geographic::Latitude;
///
/// let overflowing_latitude = Latitude::from(-5. * PI / 4.);
/// let equivalent_latitude = Latitude::from(PI / 4.);
///
/// // due precision error both values may not be exactly the same  
/// let abs_error = 0.0000000000000002;
///
/// assert!(
///     (equivalent_latitude.into_inner() - overflowing_latitude.into_inner()).abs() <= abs_error,
///     "the overflowing latitude should be as the equivalent latitude ± e"
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Latitude<T>(T);

impl<T> From<T> for Latitude<T>
where
    T: Signed + Float + FloatConst,
{
    fn from(value: T) -> Self {
        Self(if (-T::FRAC_PI_2()..=T::FRAC_PI_2()).contains(&value) {
            value
        } else {
            value.sin().asin()
        })
    }
}

impl<T> From<Cartesian<T>> for Latitude<T>
where
    T: Signed + Float + FloatConst,
{
    /// Computes the [Latitude] of the given [Cartesian] as specified by the [Spherical coordinate
    /// system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(point: Cartesian<T>) -> Self {
        let theta = match (point.x, point.y, point.z) {
            (x, y, z) if z > T::zero() => Float::atan(Float::sqrt(x.powi(2) + y.powi(2)) / z),
            (x, y, z) if z < T::zero() => {
                T::PI() + Float::atan(Float::sqrt(x.powi(2) + y.powi(2)) / z)
            }
            (x, y, z) if z == T::zero() && x * y != T::zero() => T::FRAC_PI_2(),
            // (x, y, z) if x == y && y == z => FRAC_PI_2, // fallback value
            _ => T::FRAC_PI_2(), // fallback value
        };

        (T::FRAC_PI_2() - theta).into()
    }
}

impl<T> Latitude<T> {
    /// Returns the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

/// Represents the radius in a geographic system of coordinates.
///
/// ## Definition
/// Since the altitude of a point on a sphere is the distance between that point and the center of
/// the sphere, the altitude value must be positive.
/// The absolute of any other value must be computed in order to get a proper radius notation.
///
/// ## Example
/// ```
/// use geocart::geographic::Altitude;
///
/// assert_eq!(
///     Altitude::from(-1.56),
///     Altitude::from(1.56)
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Altitude<T>(Positive<T>);

impl<T> From<T> for Altitude<T>
where
    T: Signed,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> From<Cartesian<T>> for Altitude<T>
where
    T: Signed + Float,
{
    /// Computes the [Altitude] of the given [Cartesian] as specified by the [Spherical coordinate
    /// system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(coords: Cartesian<T>) -> Self {
        (coords.x.powi(2) + coords.y.powi(2) + coords.z.powi(2))
            .sqrt()
            .into()
    }
}

impl<T> Altitude<T> {
    /// Returns the inner value.
    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }
}

/// Coordinates according to the geographical system of coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Geographic<T> {
    pub longitude: Longitude<T>,
    pub latitude: Latitude<T>,
    pub altitude: Altitude<T>,
}

impl<T> From<Cartesian<T>> for Geographic<T>
where
    T: PartialOrd + Default + Signed + Float + FloatConst + Euclid,
{
    fn from(coords: Cartesian<T>) -> Self {
        Self::default()
            .with_longitude(coords.into())
            .with_latitude(coords.into())
            .with_altitude(coords.into())
    }
}

impl<T> Geographic<T>
where
    T: Copy + Float,
{
    /// Computes the [great-circle distance](https://en.wikipedia.org/wiki/Great-circle_distance)
    /// from self to the given point (in radiants).
    pub fn distance(&self, rhs: &Self) -> T {
        let prod_latitude_sin = self.latitude.into_inner().sin() * rhs.latitude.into_inner().sin();
        let prod_latitude_cos = self.latitude.into_inner().cos() * rhs.latitude.into_inner().cos();
        let longitude_diff = (self.longitude.into_inner() - rhs.longitude.into_inner()).abs();

        (prod_latitude_sin + prod_latitude_cos * longitude_diff.cos()).acos()
    }
}

impl<T> Geographic<T> {
    pub fn with_longitude(self, longitude: Longitude<T>) -> Self {
        Self { longitude, ..self }
    }

    pub fn with_latitude(self, latitude: Latitude<T>) -> Self {
        Self { latitude, ..self }
    }

    pub fn with_altitude(self, altitude: Altitude<T>) -> Self {
        Self { altitude, ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{
        cartesian::Cartesian,
        geographic::{Altitude, Geographic, Latitude, Longitude},
        tests::approx_eq,
    };

    #[test]
    fn longitude_must_not_exceed_boundaries() {
        struct Test {
            name: &'static str,
            input: f64,
            output: f64,
        }

        vec![
            Test {
                name: "positive longitude value must not change",
                input: 1.,
                output: 1.,
            },
            Test {
                name: "negative longitude value must not change",
                input: -3.,
                output: -3.,
            },
            Test {
                name: "positive overflowing longitude must change",
                input: PI + 1.,
                output: -PI + 1.,
            },
            Test {
                name: "negative overflowing longitude must change",
                input: -PI - 1.,
                output: PI - 1.,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let longitude = Longitude::from(test.input).into_inner();

            assert_eq!(
                longitude, test.output,
                "{}: got longitude = {}, want {}",
                test.name, longitude, test.output
            );
        });
    }

    #[test]
    fn latitude_must_not_exceed_boundaries() {
        const ABS_ERROR: f64 = 0.0000000000000003;

        struct Test {
            name: &'static str,
            input: f64,
            output: f64,
        }

        vec![
            Test {
                name: "positive latitude value must not change",
                input: 1.,
                output: 1.,
            },
            Test {
                name: "negative latitude value must not change",
                input: -1.,
                output: -1.,
            },
            Test {
                name: "positive overflowing latitude must change",
                input: 7. * PI / 4.,
                output: -PI / 4.,
            },
            Test {
                name: "negative overflowing latidude must change",
                input: -7. * PI / 4.,
                output: PI / 4.,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let latitude = Latitude::from(test.input).into_inner();

            assert!(
                approx_eq(latitude, test.output, ABS_ERROR),
                "{}: got latitude = {}, want {}",
                test.name,
                latitude,
                test.output
            );
        });
    }

    #[test]
    fn geographic_from_cartesian_must_not_fail() {
        struct Test {
            name: &'static str,
            input: Cartesian<f64>,
            output: Geographic<f64>,
        }

        vec![
            Test {
                name: "north point",
                input: Cartesian::origin().with_z(1.),
                output: Geographic::default()
                    .with_latitude(Latitude::from(FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "south point",
                input: Cartesian::origin().with_z(-1.),
                output: Geographic::default()
                    .with_latitude(Latitude::from(-FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "east point",
                input: Cartesian::origin().with_y(1.),
                output: Geographic::default()
                    .with_longitude(Longitude::from(FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "weast point",
                input: Cartesian::origin().with_y(-1.),
                output: Geographic::default()
                    .with_longitude(Longitude::from(-FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "front point",
                input: Cartesian::origin().with_x(1.),
                output: Geographic::default().with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "back point",
                input: Cartesian::origin().with_x(-1.),
                output: Geographic::default()
                    .with_longitude(Longitude::from(PI))
                    .with_altitude(Altitude::from(1.)),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let point = Geographic::from(test.input);

            assert_eq!(
                point.longitude,
                test.output.longitude,
                "{}: got longitude = {}, want {}",
                test.name,
                point.longitude.into_inner(),
                test.output.longitude.into_inner(),
            );

            assert_eq!(
                point.latitude,
                test.output.latitude,
                "{}: got latitude = {}, want {}",
                test.name,
                point.latitude.into_inner(),
                test.output.latitude.into_inner(),
            );

            assert_eq!(
                point.altitude,
                test.output.altitude,
                "{}: got altitude = {}, want {}",
                test.name,
                point.altitude.into_inner(),
                test.output.altitude.into_inner(),
            );
        });
    }

    #[test]
    fn distance_must_not_fail() {
        struct Test<'a> {
            name: &'a str,
            from: Geographic<f64>,
            to: Geographic<f64>,
            distance: f64,
        }

        vec![
            Test {
                name: "Same point must be zero",
                from: Geographic::default(),
                to: Geographic::default(),
                distance: 0.,
            },
            Test {
                name: "Oposite points in the horizontal",
                from: Geographic::default(),
                to: Geographic::default().with_longitude(Longitude::from(-PI)),
                distance: PI,
            },
            Test {
                name: "Oposite points in the vertical",
                from: Geographic::default().with_latitude(Latitude::from(FRAC_PI_2)),
                to: Geographic::default().with_latitude(Latitude::from(-FRAC_PI_2)),
                distance: PI,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let distance = test.from.distance(&test.to);

            assert_eq!(
                distance, test.distance,
                "{}: distance {} ± e == {}",
                test.name, distance, test.distance,
            )
        });
    }
}
