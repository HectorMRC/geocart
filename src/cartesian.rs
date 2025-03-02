//! Cartesian system of coordinates.

use std::ops::Div;

use crate::{
    float::{Float, FRAC_PI_2, PI},
    geographic,
    transform::Transform,
};

/// Coordinates according to the cartesian system of coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Coordinates {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl From<geographic::Coordinates> for Coordinates {
    fn from(coords: geographic::Coordinates) -> Self {
        let radial_distance = match coords.altitude.as_float() {
            altitude if altitude == 0. => 1.,
            altitude => altitude,
        };

        let theta = FRAC_PI_2 - coords.latitude.as_float();
        let phi = coords.longitude.as_float();

        // improves sin & cos precision for exact numbers
        let precise_sin_cos = |rad: Float| -> (Float, Float) {
            if rad.abs() == FRAC_PI_2 {
                return (rad.signum(), 0.);
            } else if rad.abs() == PI {
                return (0., -1.);
            } else if rad == 0. {
                return (0., 1.);
            }

            (rad.sin(), rad.cos())
        };

        let (theta_sin, theta_cos) = precise_sin_cos(theta);
        let (phi_sin, phi_cos) = precise_sin_cos(phi);

        Self {
            x: radial_distance * theta_sin * phi_cos,
            y: radial_distance * theta_sin * phi_sin,
            z: radial_distance * theta_cos,
        }
    }
}

impl IntoIterator for Coordinates {
    type Item = Float;

    type IntoIter = std::array::IntoIter<Float, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y, self.z].into_iter()
    }
}

impl Div<Float> for Coordinates {
    type Output = Self;

    fn div(self, rhs: Float) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Coordinates {
    pub fn with_x(self, x: Float) -> Self {
        Self { x, ..self }
    }

    pub fn with_y(self, y: Float) -> Self {
        Self { y, ..self }
    }

    pub fn with_z(self, z: Float) -> Self {
        Self { z, ..self }
    }

    /// Returns the distance between self and the given point.
    pub fn distance(&self, rhs: &Self) -> Float {
        ((self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2) + (self.z - rhs.z).powi(2)).sqrt()
    }

    /// Performs the given transformation over self.
    pub fn transform<T: Transform<Self>>(self, transformation: T) -> Self {
        transformation.transform(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cartesian::Coordinates,
        geographic::{self, Latitude, Longitude},
        FRAC_PI_2, PI,
    };

    #[test]
    fn cartesian_from_geographic_must_not_fail() {
        struct Test {
            name: &'static str,
            input: geographic::Coordinates,
            output: Coordinates,
        }

        vec![
            Test {
                name: "north point",
                input: geographic::Coordinates::default().with_latitude(Latitude::from(FRAC_PI_2)),
                output: Coordinates::default().with_z(1.),
            },
            Test {
                name: "south point",
                input: geographic::Coordinates::default().with_latitude(Latitude::from(-FRAC_PI_2)),
                output: Coordinates::default().with_z(-1.),
            },
            Test {
                name: "east point",
                input: geographic::Coordinates::default()
                    .with_longitude(Longitude::from(FRAC_PI_2)),
                output: Coordinates::default().with_y(1.),
            },
            Test {
                name: "weast point",
                input: geographic::Coordinates::default()
                    .with_longitude(Longitude::from(-FRAC_PI_2)),
                output: Coordinates::default().with_y(-1.),
            },
            Test {
                name: "front point",
                input: geographic::Coordinates::default(),
                output: Coordinates::default().with_x(1.),
            },
            Test {
                name: "back point as negative bound",
                input: geographic::Coordinates::default().with_longitude(Longitude::from(-PI)),
                output: Coordinates::default().with_x(-1.),
            },
            Test {
                name: "back point as positive bound",
                input: geographic::Coordinates::default().with_longitude(Longitude::from(PI)),
                output: Coordinates::default().with_x(-1.),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let from = Coordinates::from(test.input);
            let point = from;
            assert_eq!(
                point, test.output,
                "{}: got cartesian point = {:#?}, want {:#?}",
                test.name, point, test.output
            );
        });
    }
}
