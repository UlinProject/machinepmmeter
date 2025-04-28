use std::ops::Deref;
use std::ptr::NonNull;

use x11::xlib;
use x11::xrecord;

#[repr(transparent)]
pub struct XRecordRange(NonNull<xrecord::XRecordRange>);

impl XRecordRange {
	/// The XRecordAllocRange function allocates and returns an XRecordRange structure.
	/// The structure is initialized to specify no protocol.
	/// The function returns NULL if the structure allocation fails.
	/// The application can free the structure by calling XFree
	pub fn new(w: impl FnOnce(&mut xrecord::XRecordRange)) -> Option<Self> {
		NonNull::new(unsafe { xrecord::XRecordAllocRange() }).map(|mut a| {
			w(unsafe { a.as_mut() });

			Self(a)
		})
	}

	#[inline]
	pub const fn as_ptr(&self) -> *mut xrecord::XRecordRange {
		self.0.as_ptr()
	}
}

impl Deref for XRecordRange {
	type Target = xrecord::XRecordRange;

	#[inline]
	fn deref(&self) -> &Self::Target {
		unsafe { self.0.as_ref() }
	}
}

impl Drop for XRecordRange {
	#[inline]
	fn drop(&mut self) {
		unsafe { xlib::XFree(self.as_ptr() as _) };
	}
}
