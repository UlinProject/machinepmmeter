pub struct ConstUppercaseData<const AVAILABLE_LEN: usize> {
	arr: [u8; AVAILABLE_LEN],
	len: usize,
}

impl<const AVAILABLE_LEN: usize> ConstUppercaseData<AVAILABLE_LEN> {
	#[inline]
	pub const fn zeroed() -> Self {
		Self {
			arr: unsafe { core::mem::zeroed() },
			len: 0,
		}
	}

	#[inline]
	pub const unsafe fn set_len(&mut self, len: usize) {
		self.len = len;
	}

	#[inline]
	pub const fn as_ptr(&self) -> *const u8 {
		self.arr.as_ptr()
	}

	#[inline]
	pub const fn as_slice(&self) -> &[u8] {
		unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
	}

	#[inline]
	pub const fn as_str(&self) -> &str {
		unsafe { core::str::from_utf8_unchecked(self.as_slice()) }
	}

	#[inline]
	pub const fn as_static_str(&'static self) -> &'static str {
		self.as_str()
	}

	#[inline]
	pub const fn len(&self) -> usize {
		self.len
	}
}

pub const fn const_ascii_uppercase<const AVAILABLE_LEN: usize>(
	instr: &'_ str,
) -> ConstUppercaseData<AVAILABLE_LEN> {
	let inarr = instr.as_bytes();
	let len = inarr.len();
	if AVAILABLE_LEN < len {
		panic!("The input array is not long enough to store the data.");
	}
	let mut out = ConstUppercaseData::zeroed();

	let mut i = 0;
	let max = len;

	while i < max {
		let a = inarr[i];
		out.arr[i] = match char::from_u32(a as _) {
			Some(m_lowcase) => m_lowcase.to_ascii_uppercase() as u8,
			None => a,
		};

		i += 1;
	}
	unsafe { out.set_len(len) };

	assert!(
		// If you are reading this, the conversion broke and you got an invalid utf-8 string.
		// I can't release a type that is not utf-8.
		//
		std::str::from_utf8(out.as_slice()).is_ok()
	);

	out
}

#[macro_export]
macro_rules! const_ascii_uppercase {
	[ $v: expr ] => {
		{
			const _IN: $crate::core::constuppercase::ConstUppercaseData<{ $v.len() }> = $crate::core::constuppercase::const_ascii_uppercase(
				$v
			);

			_IN
		}.as_static_str()
	};
}
