use crate::core::keyboard::ButtonState;
use crate::core::keyboard::err::ListenError;
use crate::core::keyboard::key::Key;
use std::ops::Deref;
use std::os::raw::{c_char, c_ulong};
use std::ptr::{NonNull, null};
use std::sync::{Mutex, OnceLock};
use x11::xlib;
use x11::xrecord::{self, XRecordInterceptData};

#[allow(clippy::type_complexity)]
static RECORD_EVENT: OnceLock<Mutex<Box<dyn FnMut(Key, ButtonState) + Sync + Send>>> =
	OnceLock::new();

pub fn xlib<T>(callback: T) -> Result<(), ListenError>
where
	T: FnMut(Key, ButtonState) + Sync + Send + 'static,
{
	// Thread display
	let raw_dpy_control = NonNull::new(unsafe { xlib::XOpenDisplay(null()) });
	let mut raw_dpy_control = match raw_dpy_control {
		Some(a) => a,
		None => return Err(ListenError::MissingDisplay),
	};
	let dpy_control = unsafe { raw_dpy_control.as_mut() };
	{
		let extension = unsafe { xlib::XInitExtension(dpy_control, c"RECORD".as_ptr()) };
		if extension.is_null() {
			return Err(ListenError::XRecordExtension);
		}
	}

	let mut record_range = {
		let mut record_range = unsafe { *xrecord::XRecordAllocRange() };
		record_range.device_events.first = xlib::KeyPress as _;
		record_range.device_events.last = xlib::KeyRelease as _;

		record_range
	};

	RECORD_EVENT
		.set(Mutex::new(Box::new(callback)))
		.map_err(|_| ListenError::AlreadyInit)?;

	// Context
	static mut __XRECORD_ALL_CLIENTS: c_ulong = xrecord::XRecordAllClients;
	let context = unsafe {
		xrecord::XRecordCreateContext(
			dpy_control,
			0,
			&raw mut __XRECORD_ALL_CLIENTS,
			1,
			&mut &mut record_range as *mut &mut xrecord::XRecordRange
				as *mut *mut xrecord::XRecordRange,
			1,
		)
	};

	if context == 0 {
		return Err(ListenError::RecordContext);
	}

	unsafe { xlib::XSync(dpy_control, 0) };
	// Run
	let result = unsafe {
		xrecord::XRecordEnableContextAsync(
			dpy_control,
			context,
			Some(record_callback),
			&mut 0,
		)
	};
	if result == 0 {
		return Err(ListenError::RecordContextEnabling);
	}
	
	loop {
		while unsafe { xlib::XPending(dpy_control) } != 0 {
			let mut event: xlib::XEvent = unsafe { std::mem::zeroed() };
			unsafe { xlib::XNextEvent(dpy_control, &mut event) };
			
			//dbg!(event);
		}
		//println!("1");
		std::thread::sleep_ms(100);
		
		//let mut event: xlib::XEvent = unsafe { std::mem::zeroed() };
		//unsafe { xlib::XNextEvent(dpy_control, &mut event) };
		//dbg!("{:?}", event);
		//unsafe { xrecord::XRecordProcessReplies(dpy_control) };
		//while unsafe { xlib::XPending(dpy_control) } > 0 {
			//let mut event: xlib::XEvent = unsafe { std::mem::zeroed() };
			//unsafe { xlib::XNextEvent(dpy_control, &mut event) };
			//dbg!("{:?}", event);
		//}
		//unsafe { xrecord::XRecordProcessReplies(dpy_control) };
		//println!("1");
		//while unsafe { xlib::XPending(dpy_control) } > 0 {
			//let mut event: xlib::XEvent = unsafe { std::mem::zeroed() };
		   //unsafe { xlib::XNextEvent(dpy_control, &mut event) };

		    
		    //dbg!("{:?}", event);
		//}
		
		//dbg!("{:?}", event);
		//unsafe { xrecord::XRecordProcessReplies(dpy_control) };
		//let mut event: xlib::XEvent = unsafe { std::mem::zeroed() };
	    //unsafe { xlib::XNextEvent(dpy_control, &mut event) };
	}
	
	unsafe { xrecord::XRecordFreeContext(dpy_control, context) };
	unsafe { xlib::XCloseDisplay(dpy_control) };
	
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

unsafe extern "C" fn record_callback(_: *mut c_char, raw_data: *mut xrecord::XRecordInterceptData) {
	#[repr(transparent)]
	struct __RawXRecordInterceptData(NonNull<XRecordInterceptData>);

	impl __RawXRecordInterceptData {
		#[inline]
		pub fn new(raw_ptr: *mut XRecordInterceptData) -> Option<Self> {
			NonNull::new(raw_ptr).map(Self)
		}
	}

	impl Deref for __RawXRecordInterceptData {
		type Target = XRecordInterceptData;

		#[inline]
		fn deref(&self) -> &Self::Target {
			unsafe { self.0.as_ref() }
		}
	}

	impl Drop for __RawXRecordInterceptData {
		fn drop(&mut self) {
			unsafe { xrecord::XRecordFreeData(self.0.as_ptr()) }
		}
	}

	if let Some(data) = __RawXRecordInterceptData::new(raw_data) {
		if let Some(record_event) = RECORD_EVENT.get() {
			if data.category != xrecord::XRecordFromServer
				|| ((data.data_len as usize) * 4 < std::mem::size_of::<_XRecordDatum>())
			{
				return;
			}

			let xdatum = NonNull::new(data.data as *mut _XRecordDatum);
			if let Some(mut xdatum) = xdatum {
				let xdatum = unsafe { xdatum.as_mut() };

				let button_state = match xdatum.r#type {
					a if a == (xlib::KeyPress as u8) => ButtonState::Pressed,
					a if a == (xlib::KeyRelease as u8) => ButtonState::Released,
					_ => return,
				};

				if let Some(key) = Key::new(xdatum.code as _) {
					(record_event.lock().unwrap_or_else(|e| e.into_inner()))(key, button_state);
				}
			}
		}
	}
}
