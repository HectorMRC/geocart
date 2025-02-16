//! Rotation transformation.

use std::ops::Neg;

use crate::{cartesian::Coordinates, radian::Radian};

use super::Transform;

/// Implements the [geometric transformation](https://en.wikipedia.org/wiki/Rotation_matrix) through which an arbitrary cartesian point can be rotated given an axis and an angle of rotation.
///
/// ## Statement
/// Being v a vector in ℝ3 and k a unit vector describing an axis of rotation about which v rotates by an angle θ, the rotation transformation rotates v according to the right hand rule.
///
/// ## Example
/// ```
/// use std::f64::consts::FRAC_PI_2;
///
/// use globe_rs::{
///     cartesian::Coordinates,
///     transform::{Rotation, Transform},
/// };
///
/// // due precision error both values may not be exactly the same
/// const ABS_ERROR: f64 = 0.0000000000000001;
///
///
/// let rotated = Rotation::default()
///     .with_axis(Coordinates::default().with_x(1.))
///     .with_theta(FRAC_PI_2.into())
///     .transform(Coordinates::default().with_y(1.));
///
/// rotated
///     .into_iter()
///     .zip(Coordinates::default().with_z(1.))
///     .for_each(|(got, want)| {
///         assert!(
///             (got - want).abs() <= ABS_ERROR,
///             "point at y1 should be rotated around the x axis to z1",
///         );
///     });
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct Rotation {
    /// The axis of rotation about which perform the transformation.
    pub axis: Coordinates,
    /// The angle of rotation.
    pub theta: Radian,
}

impl Transform<Coordinates> for Rotation {
    fn transform(&self, coords: Coordinates) -> Coordinates {
        let sin_theta = self.theta.inner().sin();
        let cos_theta = self.theta.inner().cos();
        let sub_1_cos_theta = 1. - cos_theta;

        let x = self.axis.x;
        let y = self.axis.y;
        let z = self.axis.z;

        Coordinates {
            x: coords.x * (cos_theta + x.powi(2) * sub_1_cos_theta)
                + coords.y * (x * y * sub_1_cos_theta - z * sin_theta)
                + coords.z * (x * z * sub_1_cos_theta + y * sin_theta),
            y: coords.x * (y * x * sub_1_cos_theta + z * sin_theta)
                + coords.y * (cos_theta + y.powi(2) * sub_1_cos_theta)
                + coords.z * (y * z * sub_1_cos_theta - x * sin_theta),
            z: coords.x * (z * x * sub_1_cos_theta - y * sin_theta)
                + coords.y * (z * y * sub_1_cos_theta + x * sin_theta)
                + coords.z * (cos_theta + z.powi(2) * sub_1_cos_theta),
        }
    }
}

impl Neg for Rotation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            theta: -self.theta,
            axis: self.axis,
        }
    }
}

impl Rotation {
    /// Sets as the rotation axis the normal vector pointing from the origin to the given [`Coordinates`].
    pub fn with_axis(mut self, mut coords: Coordinates) -> Self {
        let magnitude = coords.distance(&Coordinates::default());
        if magnitude != 1. {
            coords.x /= magnitude;
            coords.y /= magnitude;
            coords.z /= magnitude;
        }

        self.axis = coords;
        self
    }

    pub fn with_theta(mut self, theta: Radian) -> Self {
        self.theta = theta;
        self
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{
        cartesian::Coordinates,
        radian::Radian,
        tests::approx_eq,
        transform::{Rotation, Transform},
    };

    #[test]
    fn rotation_must_not_fail() {
        const ABS_ERROR: f64 = 0.0000000000000003;

        struct Test {
            name: &'static str,
            theta: Radian,
            axis: Coordinates,
            input: Coordinates,
            output: Coordinates,
        }

        vec![
            Test {
                name: "full rotation on the x axis must not change the y point",
                theta: Radian::from(2. * PI),
                axis: Coordinates::new(1., 0., 0.),
                input: Coordinates::new(0., 1., 0.),
                output: Coordinates::new(0., 1., 0.),
            },
            Test {
                name: "half of a whole rotation on the x axis must change the y point",
                theta: Radian::from(PI),
                axis: Coordinates::new(1., 0., 0.),
                input: Coordinates::new(0., 1., 0.),
                output: Coordinates::new(0., -1., 0.),
            },
            Test {
                name: "a quarter of a whole rotation on the x axis must change the y point",
                theta: Radian::from(FRAC_PI_2),
                axis: Coordinates::new(1., 0., 0.),
                input: Coordinates::new(0., 1., 0.),
                output: Coordinates::new(0., 0., 1.),
            },
            Test {
                name: "full rotation on the z axis must not change the y point",
                theta: Radian::from(2. * PI),
                axis: Coordinates::new(0., 0., 1.),
                input: Coordinates::new(0., 1., 0.),
                output: Coordinates::new(0., 1., 0.),
            },
            Test {
                name: "half of a whole rotation on the z axis must change the y point",
                theta: Radian::from(PI),
                axis: Coordinates::new(0., 0., 1.),
                input: Coordinates::new(0., 1., 0.),
                output: Coordinates::new(0., -1., 0.),
            },
            Test {
                name: "a quarter of a whole rotation on the z axis must change the y point",
                theta: Radian::from(FRAC_PI_2),
                axis: Coordinates::new(0., 0., 1.),
                input: Coordinates::new(0., 1., 0.),
                output: Coordinates::new(-1., 0., 0.),
            },
            Test {
                name: "rotate over itself must not change the point",
                theta: Radian::from(FRAC_PI_2),
                axis: Coordinates::new(0., 1., 0.),
                input: Coordinates::new(0., 1., 0.),
                output: Coordinates::new(0., 1., 0.),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let rotated = Rotation::default()
                .with_axis(test.axis)
                .with_theta(test.theta)
                .transform(test.input);

            rotated
                .into_iter()
                .zip(test.output)
                .for_each(|(got, want)| {
                    assert!(
                        approx_eq(got, want, ABS_ERROR),
                        "{}: got rotated = {:?}, want ± e = {:?}",
                        test.name,
                        rotated,
                        test.output
                    );
                });
        });
    }
}
