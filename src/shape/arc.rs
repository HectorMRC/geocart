//! Arc shape iterator.

use crate::{cartesian, float::Float, geographic, transform::Rotation, transform::Transform};

/// Represents the arc shape between two points in a geocart.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Arc {
    pub from: geographic::Coordinates,
    pub to: geographic::Coordinates,
    pub segments: usize,
}

impl IntoIterator for Arc {
    type Item = geographic::Coordinates;

    type IntoIter = ArcIter;

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
                theta: (dot.acos() / self.segments as Float).into(),
            },
        }
    }
}

impl Arc {
    pub fn new(segments: usize) -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
            segments,
        }
    }

    pub fn with_from(self, from: geographic::Coordinates) -> Self {
        Self { from, ..self }
    }

    pub fn with_to(self, to: geographic::Coordinates) -> Self {
        Self { to, ..self }
    }
}

/// Iterator over the [`Arc`] shape.
#[derive(Debug)]
pub struct ArcIter {
    from: cartesian::Coordinates,
    to: cartesian::Coordinates,
    total_segments: usize,
    next_segment: usize,
    rotation: Rotation,
}

impl Iterator for ArcIter {
    type Item = geographic::Coordinates;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_segment > self.total_segments {
            return None;
        }

        if self.next_segment == self.total_segments {
            return Some(self.to.into());
        }

        let next = self
            .rotation
            .with_theta(self.rotation.theta * self.next_segment as Float)
            .transform(self.from)
            .into();

        self.next_segment += 1;
        Some(next)
    }
}
