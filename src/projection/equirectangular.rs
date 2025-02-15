//! Equirectangular projection.

use crate::{cartesian, geographic, Float};

use super::Projection;

/// Equirectangular projection.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Equirectangular {
    radius: Float,
}

impl Projection for Equirectangular {
    fn forward(&self, coordinates: &geographic::Coordinates) -> cartesian::Coordinates {
        cartesian::Coordinates {
            x: self.radius * coordinates.longitude.inner(),
            y: self.radius * coordinates.latitude.inner(),
            ..Default::default()
        }
    }

    fn reverse(&self, coordinates: &cartesian::Coordinates) -> geographic::Coordinates {
        geographic::Coordinates {
            latitude: (coordinates.y / self.radius).into(),
            longitude: (coordinates.x / self.radius).into(),
            ..Default::default()
        }
    }
}
