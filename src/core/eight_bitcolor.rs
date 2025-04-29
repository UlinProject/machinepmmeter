use serde::Deserialize;

use crate::core::maybe::Maybe;
use crate::maybe;

#[derive(Clone, Copy, Debug)]
pub struct EightBitColor {
	red: u8,
	green: u8,
	blue: u8,
}

impl Default for EightBitColor {
	#[inline]
	fn default() -> Self {
		Self::new(255, 255, 255)
	}
}

impl From<(u8, u8, u8)> for EightBitColor {
	#[inline]
	fn from((r, g, b): (u8, u8, u8)) -> Self {
		Self::new(r, g, b)
	}
}

impl From<EightBitColor> for (u8, u8, u8) {
	#[inline]
	fn from(val: EightBitColor) -> Self {
		EightBitColor::into(val)
	}
}

impl From<EightBitColor> for (f64, f64, f64) {
	#[inline]
	fn from(val: EightBitColor) -> Self {
		val.into_rgb()
	}
}

impl<'de> Deserialize<'de> for EightBitColor {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let (red, green, blue): (u8, u8, u8) = Deserialize::deserialize(deserializer)?;

		Ok(EightBitColor::new(red, green, blue))
	}
}

impl EightBitColor {
	#[inline]
	pub const fn new(red: u8, green: u8, blue: u8) -> Self {
		Self { red, green, blue }
	}

	pub fn is_notblack(&self, transparent: impl Maybe<f64>) -> bool {
		self.red != 0
			|| self.green != 0
			|| self.blue != 0
			|| maybe!((transparent) {transparent != 1.0} else {false})
	}

	#[inline]
	pub const fn into(self) -> (u8, u8, u8) {
		(self.red, self.green, self.blue)
	}

	pub const fn into_rgb(self) -> (f64, f64, f64) {
		(
			(self.red as f64) / 255.0,
			(self.green as f64) / 255.0,
			(self.blue as f64) / 255.0,
		)
	}

	pub fn into_rgba(self, transparent: impl Maybe<f64>) -> (f64, f64, f64, f64) {
		let (r, g, b) = self.into_rgb();
		(
			r,
			g,
			b,
			maybe!(
				(transparent) {
					transparent
				} else {
					1.0
				}
			),
		)
	}
}
