//! Rotation transformation.

use num_traits::{Float, FloatConst, Signed};

use crate::{
    cartesian::{Cartesian, Vector},
    radian::Radian,
};

use super::Transform;

/// Implements the [geometric transformation](https://en.wikipedia.org/wiki/Rotation_matrix)
/// through which an arbitrary cartesian point can be rotated given an axis and an angle of
/// rotation.
///
/// ## Statement
/// Being v a vector in ℝ3 and k a unit vector describing an axis of rotation about which v rotates
/// by an angle θ, the rotation transformation rotates v according to the right hand rule.
///
/// ## Example
/// ```
/// use std::f64::consts::FRAC_PI_2;
///
/// use geocart::{
///     cartesian::{Cartesian, Vector},
///     transform::{Rotation, Transform},
/// };
///
/// // due precision error both values may not be exactly the same
/// const ABS_ERROR: f64 = 0.0000000000000001;
///
///
/// let rotated = Rotation::noop()
///     .with_axis(Vector::x())
///     .with_theta(FRAC_PI_2.into())
///     .transform(Cartesian::origin().with_y(1.));
///
/// rotated
///     .into_iter()
///     .zip(Cartesian::origin().with_z(1.))
///     .for_each(|(got, want)| {
///         assert!(
///             (got - want).abs() <= ABS_ERROR,
///             "point at y1 should be rotated around the x axis to z1",
///         );
///     });
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Rotation<T> {
    /// The axis of rotation about which perform the transformation.
    pub axis: Vector<T>,
    /// The angle of rotation.
    pub theta: Radian<T>,
}

impl<T> Transform<Cartesian<T>> for Rotation<T>
where
    T: Float,
{
    fn transform(&self, coords: Cartesian<T>) -> Cartesian<T> {
        let sin_theta = self.theta.into_inner().sin();
        let cos_theta = self.theta.into_inner().cos();
        let sub_1_cos_theta = T::one() - cos_theta;

        let x = self.axis.as_cartesian().x;
        let y = self.axis.as_cartesian().y;
        let z = self.axis.as_cartesian().z;

        Cartesian {
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

impl<T> Rotation<T>
where
    T: Signed + Float + FloatConst,
{
    /// Creates a rotation instance that performs no transformation.
    pub fn noop() -> Self {
        Self {
            axis: Cartesian::origin().into(),
            theta: T::zero().into(),
        }
    }
}

impl<T> Rotation<T> {
    pub fn with_axis(self, axis: Vector<T>) -> Self {
        Self { axis, ..self }
    }

    pub fn with_theta(self, theta: Radian<T>) -> Self {
        Self { theta, ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{
        cartesian::{Cartesian, Vector},
        radian::Radian,
        tests::approx_eq,
        transform::{Rotation, Transform},
    };

    #[test]
    fn rotation_must_not_fail() {
        const ABS_ERROR: f64 = 0.0000000000000003;

        struct Test {
            name: &'static str,
            theta: Radian<f64>,
            axis: Vector<f64>,
            input: Cartesian<f64>,
            output: Cartesian<f64>,
        }

        vec![
            Test {
                name: "noop rotation must not change the point",
                theta: Radian::from(0.),
                axis: Cartesian::origin().into(),
                input: Cartesian::origin().with_x(1.).with_y(2.).with_z(3.),
                output: Cartesian::origin().with_x(1.).with_y(2.).with_z(3.),
            },
            Test {
                name: "full rotation on the x axis must not change the y point",
                theta: Radian::from(2. * PI),
                axis: Vector::x(),
                input: Cartesian::origin().with_y(1.),
                output: Cartesian::origin().with_y(1.),
            },
            Test {
                name: "half of a whole rotation on the x axis must change the y point",
                theta: Radian::from(PI),
                axis: Vector::x(),
                input: Cartesian::origin().with_y(1.),
                output: Cartesian::origin().with_y(-1.),
            },
            Test {
                name: "a quarter of a whole rotation on the x axis must change the y point",
                theta: Radian::from(FRAC_PI_2),
                axis: Vector::x(),
                input: Cartesian::origin().with_y(1.),
                output: Cartesian::origin().with_z(1.),
            },
            Test {
                name: "full rotation on the z axis must not change the y point",
                theta: Radian::from(2. * PI),
                axis: Vector::z(),
                input: Cartesian::origin().with_y(1.),
                output: Cartesian::origin().with_y(1.),
            },
            Test {
                name: "half of a whole rotation on the z axis must change the y point",
                theta: Radian::from(PI),
                axis: Vector::z(),
                input: Cartesian::origin().with_y(1.),
                output: Cartesian::origin().with_y(-1.),
            },
            Test {
                name: "a quarter of a whole rotation on the z axis must change the y point",
                theta: Radian::from(FRAC_PI_2),
                axis: Vector::z(),
                input: Cartesian::origin().with_y(1.),
                output: Cartesian::origin().with_x(-1.),
            },
            Test {
                name: "rotate over itself must not change the point",
                theta: Radian::from(FRAC_PI_2),
                axis: Vector::y(),
                input: Cartesian::origin().with_y(1.),
                output: Cartesian::origin().with_y(1.),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let rotated = Rotation::noop()
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
