use crate::{cartesian, Float, FRAC_PI_2, PI};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Represents the horizontal axis in a geographic system of coordinates.
///
/// ## Definition
/// Since the longitude of a point on a sphere is the angle east (positive) or west (negative) in reference of the maridian zero, the longitude value must be in the range __[-π, +π)__.
/// Any other value will be computed in order to set its equivalent inside that range.
///
/// ### Overflow
/// Both boundaries of the longitude range are consecutive, which means that overflowing one is the same as continuing from the other one in the same direction.
///
/// ## Example
/// ```
/// use globe_rs::{Float, PI, geographic::Longitude};
///
/// assert_eq!(
///     Longitude::from(PI + 1.),
///     Longitude::from(-PI + 1.)
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Longitude(Float);

impl From<Float> for Longitude {
    fn from(value: Float) -> Self {
        Self(
            (-PI..PI)
                .contains(&value)
                .then_some(value)
                .unwrap_or_else(|| {
                    // Both boundaries of the range are consecutive, which means that
                    // overflowing one is the same as continuing from the other one
                    // in the same direction.
                    (value + PI).rem_euclid(2. * PI) - PI
                }),
        )
    }
}

impl From<Longitude> for Float {
    fn from(value: Longitude) -> Self {
        value.0
    }
}

impl From<cartesian::Coordinates> for Longitude {
    /// Computes the [Longitude] of the given [Cartesian] as specified by the [Spherical coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(point: cartesian::Coordinates) -> Self {
        match (point.x, point.y) {
            (x, y) if x > 0. => (y / x).atan(),
            (x, y) if x < 0. && y >= 0. => (y / x).atan() + PI,
            (x, y) if x < 0. && y < 0. => (y / x).atan() - PI,
            (x, y) if x == 0. && y > 0. => FRAC_PI_2,
            (x, y) if x == 0. && y < 0. => -FRAC_PI_2,
            (x, y) if x == 0. && y == 0. => 0., // fallback value

            _ => 0., // fallback value
        }
        .into()
    }
}

/// Represents the vertical axis in a geographic system of coordinates.
///
/// ## Definition
/// Since the latitude of a point on a sphere is the angle between the equatorial plane and the straight line that goes through that point and the center of the sphere, the latitude value must be in the range __\[-π/2, +π/2\]__.
/// Any other value must be computed in order to set its equivalent inside the range.
///
/// ### Overflow
/// Overflowing any of both boundaries of the latitude range behaves like moving away from that boundary and getting closer to the oposite one.
///
/// ## Example
/// ```
/// use globe_rs::{Float, PI, geographic::Latitude};
///
/// let overflowing_latitude = Latitude::from(-5. * PI / 4.);
/// let equivalent_latitude = Latitude::from(PI / 4.);
///
/// // due precision error both values may not be exactly the same  
/// let abs_error = 0.0000000000000002;
///
/// assert!(
///     (Float::from(equivalent_latitude) - Float::from(overflowing_latitude)).abs() <= abs_error,
///     "the overflowing latitude should be as the equivalent latitude ± e"
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Latitude(Float);

impl From<Float> for Latitude {
    fn from(value: Float) -> Self {
        Self(if (-FRAC_PI_2..=FRAC_PI_2).contains(&value) {
            value
        } else {
            value.sin().asin()
        })
    }
}

impl From<Latitude> for Float {
    fn from(value: Latitude) -> Self {
        value.0
    }
}

impl From<cartesian::Coordinates> for Latitude {
    /// Computes the [Latitude] of the given [Cartesian] as specified by the [Spherical coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(point: cartesian::Coordinates) -> Self {
        let theta = match (point.x, point.y, point.z) {
            (x, y, z) if z > 0. => Float::atan(Float::sqrt(x.powi(2) + y.powi(2)) / z),
            (x, y, z) if z < 0. => PI + Float::atan(Float::sqrt(x.powi(2) + y.powi(2)) / z),
            (x, y, z) if z == 0. && x * y != 0. => FRAC_PI_2,
            // (x, y, z) if x == y && y == z => FRAC_PI_2, // fallback value
            _ => FRAC_PI_2, // fallback value
        };

        (FRAC_PI_2 - theta).into()
    }
}

/// Represents the radius in a geographic system of coordinates.
///
/// ## Definition
/// Since the altitude of a point on a sphere is the distance between that point and the center of the sphere, the altitude value must be positive.
/// The absolute of any other value must be computed in order to get a proper radius notation.
///
/// ## Example
/// ```
/// use globe_rs::geographic::Altitude;
///
/// assert_eq!(
///     Altitude::from(-1.56),
///     Altitude::from(1.56)
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Altitude(Float);

impl From<Float> for Altitude {
    fn from(value: Float) -> Self {
        Self(value.abs())
    }
}

impl From<Altitude> for Float {
    fn from(value: Altitude) -> Self {
        value.0
    }
}

impl From<cartesian::Coordinates> for Altitude {
    /// Computes the [Altitude] of the given [Cartesian] as specified by the [Spherical coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(point: cartesian::Coordinates) -> Self {
        Float::sqrt(point.x.powi(2) + point.y.powi(2) + point.z.powi(2)).into()
    }
}

/// Coordinates accodring to the geographical system of coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Coordinates {
    pub longitude: Longitude,
    pub latitude: Latitude,
    pub altitude: Altitude,
}

impl From<cartesian::Coordinates> for Coordinates {
    fn from(point: cartesian::Coordinates) -> Self {
        Self::default()
            .with_longitude(point.into())
            .with_latitude(point.into())
            .with_altitude(point.into())
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Coordinates {
    pub fn with_longitude(mut self, longitude: Longitude) -> Self {
        self.longitude = longitude;
        self
    }

    pub fn with_latitude(mut self, latitude: Latitude) -> Self {
        self.latitude = latitude;
        self
    }

    pub fn with_altitude(mut self, altitude: Altitude) -> Self {
        self.altitude = altitude;
        self
    }

    /// Computes the [great-circle distance](https://en.wikipedia.org/wiki/Great-circle_distance) from self to the given point (in radiants).
    pub fn distance(&self, other: &Self) -> Float {
        let prod_latitude_sin =
            Float::from(self.latitude).sin() * Float::from(other.latitude).sin();
        let prod_latitude_cos =
            Float::from(self.latitude).cos() * Float::from(other.latitude).cos();
        let longitude_diff = (Float::from(self.longitude) - Float::from(other.longitude)).abs();

        (prod_latitude_sin + prod_latitude_cos * longitude_diff.cos()).acos()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cartesian,
        geographic::{Altitude, Coordinates, Latitude, Longitude},
        tests::approx_eq,
        Float, FRAC_PI_2, PI,
    };

    #[test]
    fn longitude_must_not_exceed_boundaries() {
        struct Test {
            name: &'static str,
            input: Float,
            output: Float,
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
            let longitude: Float = Longitude::from(test.input).into();

            assert_eq!(
                longitude, test.output,
                "{}: got longitude = {}, want {}",
                test.name, longitude, test.output
            );
        });
    }

    #[test]
    fn latitude_must_not_exceed_boundaries() {
        const ABS_ERROR: Float = 0.0000000000000003;

        struct Test {
            name: &'static str,
            input: Float,
            output: Float,
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
            let latitude: Float = Latitude::from(test.input).into();

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
            input: cartesian::Coordinates,
            output: Coordinates,
        }

        vec![
            Test {
                name: "north point",
                input: cartesian::Coordinates::new(0., 0., 1.),
                output: Coordinates::default()
                    .with_latitude(Latitude::from(FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "south point",
                input: cartesian::Coordinates::new(0., 0., -1.),
                output: Coordinates::default()
                    .with_latitude(Latitude::from(-FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "east point",
                input: cartesian::Coordinates::new(0., 1., 0.),
                output: Coordinates::default()
                    .with_longitude(Longitude::from(FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "weast point",
                input: cartesian::Coordinates::new(0., -1., 0.),
                output: Coordinates::default()
                    .with_longitude(Longitude::from(-FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "front point",
                input: cartesian::Coordinates::new(1., 0., 0.),
                output: Coordinates::default().with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "back point",
                input: cartesian::Coordinates::new(-1., 0., 0.),
                output: Coordinates::default()
                    .with_longitude(Longitude::from(PI))
                    .with_altitude(Altitude::from(1.)),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let point = Coordinates::from(test.input);

            assert_eq!(
                point.longitude,
                test.output.longitude,
                "{}: got longitude = {}, want {}",
                test.name,
                Float::from(point.longitude),
                Float::from(test.output.longitude),
            );

            assert_eq!(
                point.latitude,
                test.output.latitude,
                "{}: got latitude = {}, want {}",
                test.name,
                Float::from(point.latitude),
                Float::from(test.output.latitude),
            );

            assert_eq!(
                point.altitude,
                test.output.altitude,
                "{}: got altitude = {}, want {}",
                test.name,
                Float::from(point.altitude),
                Float::from(test.output.altitude),
            );
        });
    }

    #[test]
    fn distance_must_not_fail() {
        struct Test<'a> {
            name: &'a str,
            from: Coordinates,
            to: Coordinates,
            distance: Float,
        }

        vec![
            Test {
                name: "Same point must be zero",
                from: Coordinates::default(),
                to: Coordinates::default(),
                distance: 0.,
            },
            Test {
                name: "Oposite points in the horizontal",
                from: Coordinates::default(),
                to: Coordinates::default().with_longitude(Longitude::from(-PI)),
                distance: PI,
            },
            Test {
                name: "Oposite points in the vertical",
                from: Coordinates::default().with_latitude(Latitude::from(FRAC_PI_2)),
                to: Coordinates::default().with_latitude(Latitude::from(-FRAC_PI_2)),
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
