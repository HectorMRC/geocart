//! Cartesian system of coordinates.

use std::ops::{Add, Div, Mul};

use num_traits::{Float, FloatConst, Signed, Zero};

use crate::{geographic::Geographic, transform::Transform};

/// Coordinates according to the cartesian system of coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cartesian<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> From<Geographic<T>> for Cartesian<T>
where
    T: Signed + Float + FloatConst,
{
    fn from(coords: Geographic<T>) -> Self {
        let radial_distance = match coords.altitude.into_inner() {
            altitude if altitude == T::zero() => T::one(),
            altitude => altitude,
        };

        let theta = T::FRAC_PI_2() - coords.latitude.into_inner();
        let phi = coords.longitude.into_inner();

        // improves sin & cos precision for exact numbers
        let precise_sin_cos = |rad: T| -> (T, T) {
            if rad.abs() == T::FRAC_PI_2() {
                return (rad.signum(), T::zero());
            } else if rad.abs() == T::PI() {
                return (T::zero(), -T::one());
            } else if rad == T::zero() {
                return (T::zero(), T::one());
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

impl<T> IntoIterator for Cartesian<T> {
    type Item = T;

    type IntoIter = std::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y, self.z].into_iter()
    }
}

impl<T> Add for Cartesian<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Mul<T> for Cartesian<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div<T> for Cartesian<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> Cartesian<T>
where
    T: Float,
{
    /// Returns the distance between self and rhs.
    pub fn distance(&self, rhs: &Self) -> T {
        ((self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2) + (self.z - rhs.z).powi(2)).sqrt()
    }

    /// Returns the magnitude of the vector of self.
    pub fn magnitude(&self) -> T {
        Cartesian::origin().distance(self)
    }

    /// Returns the dot product between the vectors of self and rhs.
    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Returns the coordinates of the normalized vector of self.
    pub fn normal(self) -> Self {
        self / self.magnitude()
    }

    /// Returns the cross product between the vectors of self and rhs.
    pub fn cross(&self, rhs: &Self) -> Self {
        Cartesian::origin()
            .with_x(self.y * rhs.z - self.z * rhs.y)
            .with_y(self.z * rhs.x - self.x * rhs.z)
            .with_z(self.x * rhs.y - self.y * rhs.x)
    }
}

impl<T> Cartesian<T>
where
    T: Zero,
{
    /// Returns the cartesian origin of coordinates.
    pub fn origin() -> Self {
        Cartesian {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

impl<T> Cartesian<T> {
    pub fn with_x(self, x: T) -> Self {
        Self { x, ..self }
    }

    pub fn with_y(self, y: T) -> Self {
        Self { y, ..self }
    }

    pub fn with_z(self, z: T) -> Self {
        Self { z, ..self }
    }

    /// Performs the given transformation over self.
    pub fn transform<U: Transform<Self>>(self, transformation: U) -> Self {
        transformation.transform(self)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{
        cartesian::Cartesian,
        geographic::{Geographic, Latitude, Longitude},
    };

    #[test]
    fn cartesian_from_geographic_must_not_fail() {
        struct Test {
            name: &'static str,
            input: Geographic<f64>,
            output: Cartesian<f64>,
        }

        vec![
            Test {
                name: "north point",
                input: Geographic::origin().with_latitude(Latitude::from(FRAC_PI_2)),
                output: Cartesian::origin().with_z(1.),
            },
            Test {
                name: "south point",
                input: Geographic::origin().with_latitude(Latitude::from(-FRAC_PI_2)),
                output: Cartesian::origin().with_z(-1.),
            },
            Test {
                name: "east point",
                input: Geographic::origin().with_longitude(Longitude::from(FRAC_PI_2)),
                output: Cartesian::origin().with_y(1.),
            },
            Test {
                name: "weast point",
                input: Geographic::origin().with_longitude(Longitude::from(-FRAC_PI_2)),
                output: Cartesian::origin().with_y(-1.),
            },
            Test {
                name: "front point",
                input: Geographic::origin(),
                output: Cartesian::origin().with_x(1.),
            },
            Test {
                name: "back point as negative bound",
                input: Geographic::origin().with_longitude(Longitude::from(-PI)),
                output: Cartesian::origin().with_x(-1.),
            },
            Test {
                name: "back point as positive bound",
                input: Geographic::origin().with_longitude(Longitude::from(PI)),
                output: Cartesian::origin().with_x(-1.),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let from = Cartesian::from(test.input);
            let point = from;
            assert_eq!(
                point, test.output,
                "{}: got cartesian point = {:#?}, want {:#?}",
                test.name, point, test.output
            );
        });
    }
}
