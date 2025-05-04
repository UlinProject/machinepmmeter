use crate::core::keyboard::ButtonState;
use crate::core::keyboard::key::Key;
use crate::core::keyboard::x11::datacontaier::{ExterDataContainer, SafeDropExterDataContainer};
use crate::core::keyboard::x11::display::XDisplay;
use crate::core::keyboard::x11::record::interdata::XRecordInterceptData;
use crate::core::keyboard::x11::record::range::XRecordRange;
use libc::{fd_set, select};
use std::error::Error;
use std::ffi::CStr;
use std::fmt::Display;
use std::os::raw::c_char;
use std::ptr::{self, NonNull};
use x11::xlib::{self};
use x11::xrecord::{self};

pub mod context;
pub mod datacontaier;
pub mod display;
pub mod record {
	pub mod interdata;
	pub mod range;
}

pub fn xlib(
	key_pressrelease_event: impl FnMut(Key, ButtonState) + Sync + Send + 'static,
	success_event: impl FnOnce(),
) -> Result<(), XLibListenError> {
	// Thread display
	let mut display = XDisplay::new().ok_or(XLibListenError::MissingDisplay)?;
	let exception = c"RECORD";
	display
		.init_exeption(exception)
		.map_err(XLibListenError::InitException)?;

	let mut extern_data_container: SafeDropExterDataContainer<
		Box<dyn FnMut(Key, ButtonState) + Sync + Send + 'static>,
	> = ExterDataContainer::container(Box::new(key_pressrelease_event));

	{
		// Context
		let mut context = display
			.new_context(
				0,
				&mut [xrecord::XRecordAllClients],
				&mut [XRecordRange::new(|w_range| {
					w_range.device_events.first = xlib::KeyPress as _;
					w_range.device_events.last = xlib::KeyRelease as _;
				})
				.ok_or(XLibListenError::CreateRecordRange)?],
			)
			.map_err(|_| XLibListenError::CreateRecordContext)?;

		context.as_mut_display().sync(false);
		// Run
		let mut en = context
			.enable(Some(&mut extern_data_container), Some(raw_record_callback))
			.ok_or(XLibListenError::RecordContextEnabling)?;

		success_event();
		{
			// main loop
			let display = en.as_mut_display();
			let x11_fd = display.get_connection_number();
			// Thank you!
			// https://www.linuxquestions.org/questions/showthread.php?p=2431345#post2431345
			//
			loop {
				while display.pending() > 0 {}
				let mut in_fds: fd_set = unsafe { std::mem::zeroed() };
				unsafe { libc::FD_ZERO(&mut in_fds) };
				unsafe { libc::FD_SET(x11_fd, &mut in_fds) };

				let select_result = unsafe {
					select(
						x11_fd + 1,
						&raw mut in_fds,
						ptr::null_mut(),
						ptr::null_mut(),
						ptr::null_mut(),
					)
				};

				if select_result == -1 {
					break;
				}
			}
		}
		drop(en);
		drop(context);
		drop(display);
	}
	drop(extern_data_container);

	Ok(())
}

#[derive(Debug)]
#[repr(C)]
struct _XRecordDatum {
	r#type: u8,
	code: u8,
	rest: u64,
	_1: bool,
	_2: bool,
	_3: bool,
	root_x: i16,
	root_y: i16,
	event_x: i16,
	event_y: i16,
	state: u16,
}

unsafe extern "C" fn raw_record_callback(
	ptr: *mut c_char,
	raw_recorddata: *mut xrecord::XRecordInterceptData,
) {
	if let Some(recorddata) = XRecordInterceptData::new(raw_recorddata) {
		if let Some(callback) = NonNull::new(
			ptr as *mut ExterDataContainer<
				Box<dyn FnMut(Key, ButtonState) + Sync + Send + 'static>,
			>,
		) {
			let callback = unsafe { callback.as_ref() };
			callback.check_and_lock(|callback| {
				record_callback(callback, &recorddata);
			});
		}
	}
}

fn record_callback(
	callback: &mut Box<dyn FnMut(Key, ButtonState) + Send + Sync + 'static>,
	data: &XRecordInterceptData,
) {
	if data.category != xrecord::XRecordFromServer
		|| ((data.data_len as usize) * 4 < std::mem::size_of::<_XRecordDatum>())
	{
		return;
	}

	let xdatum = NonNull::new(data.data as *mut _XRecordDatum);
	if let Some(xdatum) = xdatum {
		let xdatum = unsafe { xdatum.as_ref() };

		let button_state = match xdatum.r#type {
			a if a == (xlib::KeyPress as u8) => ButtonState::Pressed,
			a if a == (xlib::KeyRelease as u8) => ButtonState::Released,
			_ => return,
		};

		if let Some(key) = Key::new(xdatum.code as _) {
			(callback)(key, button_state);
		}
	}
}

#[derive(Debug)]
pub enum XLibListenError {
	MissingDisplay,
	RecordContextEnabling,
	CreateRecordContext,
	CreateRecordRange,
	InitException(#[allow(dead_code)] &'static CStr),
}

impl Display for XLibListenError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::MissingDisplay => write!(f, "MissingDisplay"),
			Self::RecordContextEnabling => write!(f, "RecordContextEnabling"),
			Self::CreateRecordContext => write!(f, "CreateRecordContext"),
			Self::CreateRecordRange => write!(f, "CreateRecordRange"),
			Self::InitException(a) => write!(f, "InitException({})", a.to_string_lossy()),
		}
	}
}

impl Error for XLibListenError {}
