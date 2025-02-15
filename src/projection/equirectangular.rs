//! Equirectangular projection.

use crate::{cartesian, geographic};

use super::Projection;

/// Equirectangular projection.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Equirectangular;

impl Projection for Equirectangular {
    fn forward(coordinates: &geographic::Coordinates) -> cartesian::Coordinates {
        cartesian::Coordinates {
            x: coordinates.longitude.inner(),
            y: coordinates.latitude.inner(),
            ..Default::default()
        }
    }

    fn reverse(coordinates: &cartesian::Coordinates) -> geographic::Coordinates {
        geographic::Coordinates {
            latitude: coordinates.y.into(),
            longitude: coordinates.x.into(),
            ..Default::default()
        }
    }
}
