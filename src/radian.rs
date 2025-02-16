//! Radian unit.

use std::ops::Neg;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{Float, TAU};

/// The [radian](https://en.wikipedia.org/wiki/Radian) unit, which is always a positive number within the range of [0, 2π].
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Radian(Float);

impl From<Float> for Radian {
    fn from(value: Float) -> Self {
        if (0. ..TAU).contains(&value) {
            return Self(value);
        }

        let mut modulus = value % TAU;
        if value.is_sign_negative() {
            modulus = (modulus + TAU) % TAU;
        }

        Self(modulus)
    }
}

impl From<Radian> for Float {
    fn from(value: Radian) -> Self {
        value.0
    }
}

impl Neg for Radian {
    type Output = Self;

    fn neg(self) -> Self::Output {
        (-self.0).into()
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Radian {
    /// Returns the inner value.
    pub fn inner(&self) -> Float {
        self.0
    }

    /// Returns the absolute difference between self and the given radiant.
    pub fn abs_diff(self, rhs: Self) -> Self {
        Self((self.0 - rhs.0).abs())
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI, TAU};

    use crate::{radian::Radian, Float};

    #[test]
    fn radiant_must_not_exceed_boundaries() {
        struct Test {
            name: &'static str,
            input: Float,
            output: Float,
        }

        vec![
            Test {
                name: "radiant within range must not change",
                input: PI,
                output: PI,
            },
            Test {
                name: "2π radiants must equals zero",
                input: TAU,
                output: 0.,
            },
            Test {
                name: "negative radiant must change",
                input: -FRAC_PI_2,
                output: TAU - FRAC_PI_2,
            },
            Test {
                name: "overflowing radiant must change",
                input: TAU + FRAC_PI_2,
                output: FRAC_PI_2,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let radiant = Radian::from(test.input).inner();

            assert_eq!(
                radiant, test.output,
                "{}: got radiant = {}, want {}",
                test.name, radiant, test.output
            );
        });
    }
}
