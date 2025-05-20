//! Equirectangular projection.

use num_traits::{Euclid, Float, FloatConst, Signed};

use crate::{cartesian::Cartesian, geographic::Geographic, positive::Positive};

use super::Projection;

/// The [equirectangular projection](https://en.wikipedia.org/wiki/Equirectangular_projection).
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Equirectangular<T> {
    /// The radius of the globe being projected.
    pub radius: Positive<T>,
}

impl<T> Projection<T> for Equirectangular<T>
where
    T: Default + PartialOrd + Signed + Float + FloatConst + Euclid,
{
    fn forward(&self, coordinates: &Geographic<T>) -> Cartesian<T> {
        Cartesian {
            x: self.radius.into_inner() * coordinates.longitude.into_inner(),
            y: self.radius.into_inner() * coordinates.latitude.into_inner(),
            ..Default::default()
        }
    }

    fn reverse(&self, coordinates: &Cartesian<T>) -> Geographic<T> {
        Geographic {
            latitude: (coordinates.y / self.radius.into_inner()).into(),
            longitude: (coordinates.x / self.radius.into_inner()).into(),
            ..Default::default()
        }
    }
}
