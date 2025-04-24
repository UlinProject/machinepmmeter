use std::{
	cell::{Ref, RefCell},
	rc::Rc,
};

use crate::__gen_transparent_gtk_type;
use gtk::{DrawingArea, ffi::GtkDrawingArea, traits::WidgetExt};

#[repr(transparent)]
#[derive(Debug)]
pub struct ViColorBlock(DrawingArea);

__gen_transparent_gtk_type! {
	#[sys(GtkDrawingArea)]
	ViColorBlock(
		new |a: DrawingArea| {
			Self(a)
		},
		ref |sself| {
			&sself.0
		},
		into |sself| {
			sself.0
		},
	)
}

impl ViColorBlock {
	pub fn new(width: i32, height: i32) -> Self {
		let drawing_area = DrawingArea::new();
		drawing_area.set_size_request(width, height);

		Self(drawing_area)
	}

	pub fn connect_background<const ALLOW_ONEDRAW_OPTIMIZE: bool>(
		self,
		red: f64,
		green: f64,
		blue: f64,
		alpha: f64,
	) -> Self {
		self.0.connect_draw(move |da, cr| {
			let allocation = da.allocation();

			cr.set_source_rgba(red, green, blue, alpha);
			cr.rectangle(0.0, 0.0, allocation.width() as _, allocation.height() as _);
			cr.fill();

			ALLOW_ONEDRAW_OPTIMIZE.into()
		});

		self
	}

	pub fn connect_state_background<const ALLOW_ONEDRAW_OPTIMIZE: bool>(
		self,
		rcptr: Rc<RefCell<(f64, f64, f64, f64)>>,
	) -> Self {
		self.0.connect_draw(move |da, cr| {
			let rcptr = rcptr.clone();
			let allocation = da.allocation();

			let rgba = {
				let read: Ref<(f64, f64, f64, f64)> = RefCell::borrow(&rcptr);
				*read
			};
			cr.set_source_rgba(rgba.0, rgba.1, rgba.2, rgba.3);
			cr.rectangle(0.0, 0.0, allocation.width() as _, allocation.height() as _);
			cr.fill();

			ALLOW_ONEDRAW_OPTIMIZE.into()
		});

		self
	}
}
