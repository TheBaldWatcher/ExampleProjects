use std::borrow::Cow;
use std::ops::{Add, Bound, Mul, RangeBounds};
use std::process::Output;

/////////////// Rgb Float /////////
#[derive(Debug, Clone, Default)]
pub struct RgbFloat {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RgbFloat {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        // TODO check0to1
        Self { r, g, b }
    }
}

/////////////// Rgb Int /////////
#[derive(Debug, Clone, Default)]
pub struct RgbInt {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbInt {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        // TODO check0to1
        Self { r, g, b }
    }
}

/////////// RgbFloat <-> RbgInt ///////
impl From<&RgbFloat> for RgbInt {
    fn from(c: &RgbFloat) -> Self {
        // TODO  #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        Self::new(
            (c.r * 255.0) as u8,
            (c.g * 255.0) as u8,
            (c.b * 255.0) as u8,
        )
    }
}

impl From<&RgbInt> for RgbFloat {
    fn from(c: &RgbInt) -> Self {
        // TODO  #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        Self::new(c.r as f64 / 255.0, c.g as f64 / 255.0, c.b as f64 / 255.0)
    }
}

///////////////// Color //////////
#[derive(Debug, Clone)]
pub enum Color {
    RgbF(RgbFloat),
    RgbI(RgbInt),
}

impl Default for Color {
    fn default() -> Self {
        Self::RgbF(RgbFloat::default())
    }
}

impl Color {
    // TODO #[must_use]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self::RgbI(RgbInt::new(r, g, b))
    }
    pub fn newf(r: f64, g: f64, b: f64) -> Self {
        Self::RgbF(RgbFloat::new(r, g, b))
    }

    pub fn int_form(&self) -> Cow<'_, RgbInt> {
        match self {
            Self::RgbI(c) => Cow::Borrowed(c),
            Self::RgbF(c) => Cow::Owned(c.into()),
        }
    }
    pub fn float_form(&self) -> Cow<'_, RgbFloat> {
        match self {
            Self::RgbI(c) => Cow::Owned(c.into()),
            Self::RgbF(c) => Cow::Borrowed(c),
        }
    }

    pub fn mix(&self, ratial: f64, rhs: Self) -> Self {
        let r = ratial.max(0.0).min(1.0);
        self * r + (1.0 - r) * rhs
    }

    fn clamp<R: RangeBounds<f64>>(val: f64, range: R) -> f64 {
        let start = match range.start_bound() {
            Bound::Included(&x) | Bound::Excluded(&x) => x,
            _ => std::f64::NEG_INFINITY,
        };
        let end = match range.end_bound() {
            Bound::Included(&x) | Bound::Excluded(&x) => x,
            _ => std::f64::INFINITY,
        };
        if start > val {
            start
        } else if val > end {
            end
        } else {
            val
        }
    }
}

////////// Add //////////
impl Add<&Color> for &Color {
    type Output = Color;
    fn add(self, rhs: &Color) -> Self::Output {
        let f_lhs = self.float_form();
        let f_rhs = rhs.float_form();
        Color::newf(
            Color::clamp((f_lhs.r + f_rhs.r) / 2.0, 0.0..=1.0),
            Color::clamp((f_lhs.g + f_rhs.g) / 2.0, 0.0..=1.0),
            Color::clamp((f_lhs.b + f_rhs.b) / 2.0, 0.0..=1.0),
        )
    }
}

impl Add<Color> for &Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        self + &rhs
    }
}

impl Add<&Color> for Color {
    type Output = Color;
    fn add(self, rhs: &Color) -> Self::Output {
        &self + rhs
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        &self + &rhs
    }
}

////////// Mul //////////
impl Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        let f = self.float_form();
        Color::newf(
            Color::clamp(f.r * rhs, 0.0..=1.0),
            Color::clamp(f.g * rhs, 0.0..=1.0),
            Color::clamp(f.b * rhs, 0.0..=1.0),
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        &rhs * self
    }
}

////////// UT //////////
#[test]
fn test_mul() {
    {
        let c = Color::newf(1.0, 1.0, 1.0);
        assert_eq!((c * 0.5).int_form().r, 127);
    }
    {
        let c = Color::newf(1.0, 1.0, 1.0);
        assert_eq!((&c * 0.4).int_form().r, 102);
    }
    {
        let c = Color::newf(1.0, 1.0, 1.0);
        assert_eq!((0.4 * &c).int_form().r, 102);
    }
    {
        let c = Color::newf(1.0, 1.0, 1.0);
        assert_eq!((0.5 * c).int_form().r, 127);
    }
}

#[test]
fn test_add() {
    let c_1 = Color::newf(0.0, 0.0, 0.0);
    let c_2 = Color::newf(1.0, 1.0, 1.0);
    assert_eq!((&c_1 + &c_2).int_form().r, 127);
    assert_eq!((&c_1 + &c_2).int_form().g, 127);
    assert_eq!((&c_1 + &c_2).int_form().b, 127);
}
