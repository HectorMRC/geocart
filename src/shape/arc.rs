//! Arc shape iterator.

use std::num::NonZeroUsize;

use num_traits::{Euclid, Float, FloatConst, Signed};

use crate::{cartesian, geographic, transform::Rotation, transform::Transform};

/// Represents the arc shape between two points in a geocart.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Arc<T> {
    pub from: geographic::Coordinates<T>,
    pub to: geographic::Coordinates<T>,
    pub segments: usize,
}

impl<T> IntoIterator for Arc<T>
where
    T: Default + PartialOrd + Signed + Float + FloatConst + Euclid,
{
    type Item = geographic::Coordinates<T>;

    type IntoIter = ArcIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let from = cartesian::Coordinates::from(self.from);
        let to = cartesian::Coordinates::from(self.to);

        let cross = cartesian::Coordinates {
            x: from.y * to.z - from.z * to.y,
            y: from.z * to.x - from.x * to.z,
            z: from.x * to.y - from.y * to.x,
        };

        let dot = from.x * to.x + from.y * to.y + from.z * to.z;

        ArcIter {
            from,
            to,
            total_segments: self.segments,
            next_segment: 0,
            rotation: Rotation {
                axis: cross / cross.distance(&Default::default()),
                theta: T::from(self.segments)
                    .map(|theta| dot.acos() / theta)
                    .unwrap_or_default()
                    .into(),
            },
        }
    }
}

impl<T> Arc<T>
where
    T: Default,
{
    pub fn new(segments: NonZeroUsize) -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
            segments: segments.get(),
        }
    }
}

impl<T> Arc<T> {
    pub fn with_from(self, from: geographic::Coordinates<T>) -> Self {
        Self { from, ..self }
    }

    pub fn with_to(self, to: geographic::Coordinates<T>) -> Self {
        Self { to, ..self }
    }
}

/// Iterator over the [`Arc`] shape.
#[derive(Debug)]
pub struct ArcIter<T> {
    from: cartesian::Coordinates<T>,
    to: cartesian::Coordinates<T>,
    total_segments: usize,
    next_segment: usize,
    rotation: Rotation<T>,
}

impl<T> Iterator for ArcIter<T>
where
    T: Default + PartialOrd + Signed + Float + FloatConst + Euclid,
{
    type Item = geographic::Coordinates<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_segment > self.total_segments {
            return None;
        }

        if self.next_segment == self.total_segments {
            return Some(self.to.into());
        }

        let next = self
            .rotation
            .with_theta(self.rotation.theta * T::from(self.next_segment)?)
            .transform(self.from)
            .into();

        self.next_segment += 1;
        Some(next)
    }
}
