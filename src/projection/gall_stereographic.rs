//! Gall Stereographic projection.

use num_traits::{Euclid, Float, FloatConst, Signed};

use crate::{cartesian, float::PositiveFloat, geographic};

use super::Projection;

/// The [Gall Stereographic projection](https://en.wikipedia.org/wiki/Gall_stereographic_projection).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GallStereographic<T> {
    radius: PositiveFloat<T>,
}

impl<T> Projection<T> for GallStereographic<T>
where
    T: Default + PartialOrd + Signed + Float + FloatConst + Euclid,
{
    fn forward(&self, coordinates: &geographic::Coordinates<T>) -> cartesian::Coordinates<T> {
        let two = T::one() + T::one();

        cartesian::Coordinates {
            x: self.radius.into_inner() * coordinates.longitude.into_inner() / T::SQRT_2(),
            y: self.radius.into_inner()
                * (T::one() + T::SQRT_2() / two)
                * (coordinates.latitude.into_inner() / two).tan(),
            ..Default::default()
        }
    }

    fn reverse(&self, coordinates: &cartesian::Coordinates<T>) -> geographic::Coordinates<T> {
        let two = T::one() + T::one();

        geographic::Coordinates {
            latitude: (two
                * (coordinates.y / (self.radius.into_inner() * (T::one() + T::SQRT_2() / two)))
                    .atan())
            .into(),
            longitude: (coordinates.x * T::SQRT_2() / self.radius.into_inner()).into(),
            ..Default::default()
        }
    }
}
