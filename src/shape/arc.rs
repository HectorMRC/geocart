//! Arc shape iterator.

use std::num::NonZeroUsize;

use num_traits::{Euclid, Float, FloatConst, Signed};

use crate::{
    cartesian::{self, Cartesian, Vector},
    geographic::Geographic,
    transform::{Rotation, Transform},
};

/// Represents the arc shape between two points in a globe.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Arc<T> {
    /// The initial endpoint of the arc.
    pub from: Geographic<T>,
    /// The final endpoint of the arc.
    pub to: Geographic<T>,
    /// The total amount of segments (straight lines) the arc is made of.
    pub segments: NonZeroUsize,
}

impl<T> IntoIterator for Arc<T>
where
    T: Default + PartialOrd + Signed + Float + FloatConst + Euclid,
{
    type Item = Geographic<T>;

    type IntoIter = ArcIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let from = Cartesian::from(self.from);
        let to = Cartesian::from(self.to);

        let rotation = {
            let from = Vector::from(from);
            let to = Vector::from(to);

            Rotation::noop().with_axis(from.cross(&to)).with_theta(
                T::from(self.segments.get())
                    .map(|segments| {
                        (from.dot(&to) / (from.magnitude() * to.magnitude())).acos() / segments
                    })
                    .unwrap_or_default()
                    .into(),
            )
        };

        ArcIter {
            from,
            to,
            total_segments: self.segments.get(),
            next_segment: 0,
            rotation,
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
            segments,
        }
    }
}

impl<T> Arc<T> {
    pub fn with_from(self, from: Geographic<T>) -> Self {
        Self { from, ..self }
    }

    pub fn with_to(self, to: Geographic<T>) -> Self {
        Self { to, ..self }
    }
}

/// Iterator over the [`Arc`] shape.
#[derive(Debug)]
pub struct ArcIter<T> {
    from: cartesian::Cartesian<T>,
    to: cartesian::Cartesian<T>,
    total_segments: usize,
    next_segment: usize,
    rotation: Rotation<T>,
}

impl<T> Iterator for ArcIter<T>
where
    T: Default + PartialOrd + Signed + Float + FloatConst + Euclid,
{
    type Item = Geographic<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_segment > self.total_segments {
            return None;
        }

        if self.next_segment == self.total_segments {
            return Some(self.to.into());
        }

        let next = Rotation::noop()
            .with_axis(self.rotation.axis)
            .with_theta(self.rotation.theta * T::from(self.next_segment)?)
            .transform(self.from)
            .into();

        self.next_segment += 1;
        Some(next)
    }
}
