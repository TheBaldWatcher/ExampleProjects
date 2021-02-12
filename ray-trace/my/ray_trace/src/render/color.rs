use std::borrow::Cow;

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
}
