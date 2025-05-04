use crate::{__gen_transparent_gtk_type, app::config::ColorAppConfig};
use gtk::{
	DrawingArea,
	ffi::GtkDrawingArea,
	traits::{StyleContextExt, WidgetExt},
};
use std::{
	cell::{Ref, RefCell},
	rc::Rc,
};

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
		drawing_area.style_context().add_class("vicolorblock");
		drawing_area.set_size_request(width, height);
		drawing_area.set_visible(true);

		Self(drawing_area)
	}

	pub fn connect_color<const ALLOW_ONEDRAW_OPTIMIZE: bool>(
		self,
		c_app_config: impl AsRef<ColorAppConfig>,

		get_color: impl FnOnce(&ColorAppConfig) -> (u8, u8, u8),
		alpha: f64,
	) -> Self {
		let (red, green, blue) = get_color(c_app_config.as_ref());

		self.connect_background::<ALLOW_ONEDRAW_OPTIMIZE>(red, green, blue, alpha)
	}

	pub fn connect_background<const ALLOW_ONEDRAW_OPTIMIZE: bool>(
		self,
		red: u8,
		green: u8,
		blue: u8,
		alpha: f64,
	) -> Self {
		self.0.connect_draw(move |da, cr| {
			let allocation = da.allocation();

			cr.set_source_rgba(
				(red as f64) / 255.0,
				(green as f64) / 255.0,
				(blue as f64) / 255.0,
				alpha,
			);
			cr.rectangle(
				0.0,
				0.0,
				allocation.width().into(),
				allocation.height().into(),
			);
			let _ = cr.fill();

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
			let _e = cr.fill();

			ALLOW_ONEDRAW_OPTIMIZE.into()
		});

		self
	}
}
