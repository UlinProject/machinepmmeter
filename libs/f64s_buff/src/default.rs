
use std::fmt::Write;
use std::ops::Deref;

#[repr(transparent)]
pub struct F64SBuff(String);

impl F64SBuff {
	#[inline]
	pub fn with_capacity(len: usize) -> Self {
		Self(String::with_capacity(len))
	}

	#[inline]
	pub fn new() -> Self {
		Self::with_capacity(24)
	}

	#[allow(dead_code)]
	#[inline]
	pub const fn empty() -> Self {
		Self(String::new())
	}

	#[allow(dead_code)]
	pub fn format_and_get(&mut self, v: f64) -> F64SBuffAutoClear {
		let _e = write!(&mut self.0, "{v}");

		F64SBuffAutoClear(self)
	}
}

impl Default for F64SBuff {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

#[repr(transparent)]
pub struct F64SBuffAutoClear<'a>(&'a mut F64SBuff);

impl Deref for F64SBuffAutoClear<'_> {
	type Target = str;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0.0
	}
}

impl Drop for F64SBuffAutoClear<'_> {
	#[inline]
	fn drop(&mut self) {
		self.0.0.clear();
	}
}
