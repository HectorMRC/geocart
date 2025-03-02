//! Arc shape iterator.

use crate::{cartesian, float::Float, geographic, transform::Rotation};

/// Represents the arc shape between two points in a globe.
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
            rotation: Rotation {
                axis: cross / cross.distance(&Default::default()),
                theta: (dot.acos() / self.segments as Float).into(),
            },
            current_segment: 0,
            total_segments: self.segments,
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
#[derive(Debug, Clone, Copy)]
pub struct ArcIter {
    from: cartesian::Coordinates,
    to: cartesian::Coordinates,
    current_segment: usize,
    total_segments: usize,
    rotation: Rotation,
}

impl Iterator for ArcIter {
    type Item = geographic::Coordinates;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_segment > self.total_segments || self.total_segments == 0 {
            return None;
        }

        let segment = self.current_segment;
        self.current_segment += 1;

        if self.current_segment == 0 {
            return Some(self.from.into());
        }

        if self.current_segment == self.total_segments {
            return Some(self.to.into());
        }

        let mut rotation = self.rotation;
        rotation.theta *= segment as Float;

        Some(self.from.transform(rotation).into())
    }
}
