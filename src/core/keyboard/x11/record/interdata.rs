use std::ops::Deref;
use std::ptr::NonNull;
use x11::xrecord;

#[repr(transparent)]
pub struct XRecordInterceptData(NonNull<xrecord::XRecordInterceptData>);

impl XRecordInterceptData {
	#[inline]
	pub fn new(raw_ptr: *mut xrecord::XRecordInterceptData) -> Option<Self> {
		NonNull::new(raw_ptr).map(Self)
	}
}

impl Deref for XRecordInterceptData {
	type Target = xrecord::XRecordInterceptData;

	#[inline]
	fn deref(&self) -> &Self::Target {
		unsafe { self.0.as_ref() }
	}
}

impl Drop for XRecordInterceptData {
	#[inline]
	fn drop(&mut self) {
		unsafe { xrecord::XRecordFreeData(self.0.as_ptr()) }
	}
}
