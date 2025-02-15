use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{geographic, Float, FRAC_PI_2, PI};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Coordinates according to the cartesian system of coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Coordinates {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Neg for Coordinates {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign for Coordinates {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Float> for Coordinates {
    type Output = Self;

    fn mul(mut self, rhs: Float) -> Self::Output {
        self /= rhs;
        self
    }
}

impl MulAssign<Float> for Coordinates {
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<Float> for Coordinates {
    type Output = Self;

    fn div(mut self, rhs: Float) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<Float> for Coordinates {
    fn div_assign(&mut self, rhs: Float) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl From<geographic::Coordinates> for Coordinates {
    fn from(point: geographic::Coordinates) -> Self {
        let radial_distance = match point.altitude.into() {
            altitude if altitude == 0. => 1.,
            altitude => altitude,
        };

        let theta = FRAC_PI_2 - Float::from(point.latitude);
        let phi = point.longitude.into();

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

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Coordinates {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn with_x(mut self, x: Float) -> Self {
        self.x = x;
        self
    }

    pub fn with_y(mut self, y: Float) -> Self {
        self.y = y;
        self
    }

    pub fn with_z(mut self, z: Float) -> Self {
        self.z = z;
        self
    }

    /// Returns the distance between self and the given point.
    pub fn distance(&self, rhs: &Self) -> Float {
        ((self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2) + (self.z - rhs.z).powi(2)).sqrt()
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
                output: Coordinates::new(0., 0., 1.),
            },
            Test {
                name: "south point",
                input: geographic::Coordinates::default().with_latitude(Latitude::from(-FRAC_PI_2)),
                output: Coordinates::new(0., 0., -1.),
            },
            Test {
                name: "east point",
                input: geographic::Coordinates::default()
                    .with_longitude(Longitude::from(FRAC_PI_2)),
                output: Coordinates::new(0., 1., 0.),
            },
            Test {
                name: "weast point",
                input: geographic::Coordinates::default()
                    .with_longitude(Longitude::from(-FRAC_PI_2)),
                output: Coordinates::new(0., -1., 0.),
            },
            Test {
                name: "front point",
                input: geographic::Coordinates::default(),
                output: Coordinates::new(1., 0., 0.),
            },
            Test {
                name: "back point as negative bound",
                input: geographic::Coordinates::default().with_longitude(Longitude::from(-PI)),
                output: Coordinates::new(-1., 0., 0.),
            },
            Test {
                name: "back point as positive bound",
                input: geographic::Coordinates::default().with_longitude(Longitude::from(PI)),
                output: Coordinates::new(-1., 0., 0.),
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
