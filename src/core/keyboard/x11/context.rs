use x11::xrecord;

use crate::core::keyboard::ButtonState;
use crate::core::keyboard::key::Key;
use crate::core::keyboard::x11::datacontaier::SafeDropExternDataContainer;
use crate::core::keyboard::x11::display::XDisplay;
use std::num::NonZeroI32;
use std::os::raw::c_char;
use std::ptr::null_mut;

pub struct XRecordContext<'a> {
	display: &'a mut XDisplay,
	ident: u64,
}

pub struct XRecordEnContext<'context, 'display> {
	context: &'context mut XRecordContext<'display>,
	ident: NonZeroI32,
}

impl<'display> XRecordEnContext<'_, 'display> {
	#[allow(dead_code)]
	#[inline]
	pub const fn as_mut_context(&mut self) -> &mut XRecordContext<'display> {
		self.context
	}

	#[inline]
	pub const fn as_mut_display(&mut self) -> &mut XDisplay {
		self.context.as_mut_display()
	}

	#[allow(dead_code)]
	pub fn disable(self) {}
}

impl Drop for XRecordEnContext<'_, '_> {
	fn drop(&mut self) {
		unsafe {
			xrecord::XRecordDisableContext(self.as_mut_display().as_ptr(), self.ident.get() as _);
		}
	}
}

impl<'display> XRecordContext<'display> {
	#[inline]
	pub const unsafe fn from_raw(display: &'display mut XDisplay, ident: u64) -> Self {
		Self { display, ident }
	}

	#[allow(clippy::type_complexity)] // TODO REFACTORING ME
	pub fn enable<'context>(
		&'context mut self,
		data_ptr: Option<
			&mut SafeDropExternDataContainer<
				Box<dyn FnMut(Key, ButtonState) + Sync + Send + 'static>,
			>,
		>,
		trig_fn: Option<unsafe extern "C" fn(*mut c_char, *mut xrecord::XRecordInterceptData)>,
	) -> Option<XRecordEnContext<'context, 'display>> {
		let result = unsafe {
			xrecord::XRecordEnableContextAsync(
				self.display.as_ptr(),
				self.ident,
				trig_fn,
				data_ptr.map(|a| a.as_ptr() as _).unwrap_or(null_mut()),
			)
		};

		NonZeroI32::new(result).map(move |ident| XRecordEnContext {
			ident,
			context: self,
		})
	}

	#[inline]
	pub const fn as_mut_display(&mut self) -> &mut XDisplay {
		self.display
	}
}

impl Drop for XRecordContext<'_> {
	fn drop(&mut self) {
		unsafe { xrecord::XRecordFreeContext(self.display.as_ptr(), self.ident) };
	}
}
