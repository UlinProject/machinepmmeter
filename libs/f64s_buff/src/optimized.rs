use std::ops::Deref;

#[repr(transparent)]
pub struct F64SBuff(ryu::Buffer);

impl F64SBuff {
	#[inline]
	pub fn new() -> Self {
		Self(ryu::Buffer::new())
	}

	#[inline]
	pub fn format_and_get(&mut self, v: f64) -> F64SBuffAutoClear {
		F64SBuffAutoClear(self.0.format(v))
	}
}

impl Default for F64SBuff {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

// makes almost no sense but allows you to exclude clippy warnings (clippy::needless_borrow)
// when switching `f64_string_optimized`
#[repr(transparent)]
pub struct F64SBuffAutoClear<'a>(&'a str);

impl Deref for F64SBuffAutoClear<'_> {
	type Target = str;

	#[inline]
	fn deref(&self) -> &Self::Target {
		self.0
	}
}

impl Drop for F64SBuffAutoClear<'_> {
	#[inline]
	fn drop(&mut self) {}
}
