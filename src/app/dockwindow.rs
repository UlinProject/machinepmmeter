use crate::{__gen_transparent_gtk_type, app::config::WindowAppConfig, core::maybe::Maybe, maybe};
use gtk::{
	Application, ApplicationWindow, cairo,
	ffi::GtkApplicationWindow,
	gdk::{Monitor, WindowTypeHint, traits::MonitorExt},
	traits::{BinExt, GtkWindowExt, StyleContextExt, WidgetExt},
};
use log::trace;
use serde::Deserialize;
use std::ops::Deref;

#[repr(transparent)]
#[derive(Debug)]
pub struct AppViDockWindow(ApplicationWindow);

__gen_transparent_gtk_type! {
	#[sys(GtkApplicationWindow)]
	AppViDockWindow(
		new |a: ApplicationWindow| {
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

impl Deref for AppViDockWindow {
	type Target = ApplicationWindow;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl AppViDockWindow {
	pub fn new(
		app: &'_ Application,
		title: &'_ str,
		w_app_config: impl AsRef<WindowAppConfig>,
	) -> Self {
		let w_app_config = w_app_config.as_ref();
		let window = ApplicationWindow::new(app);
		window.style_context().add_class("appvidockwindow");
		window.set_visible(false);
		window.set_title(title);
		window.set_decorated(false);
		window.set_app_paintable(true);
		window.set_type_hint(WindowTypeHint::Dock);
		window.set_default_size(
			w_app_config.get_width_or_default(),
			w_app_config.get_height_or_default(),
		);
		window.set_keep_above(true);

		Self(window)
	}

	pub fn connect_transparent_background(&self, start_height: impl Maybe<f64>, alpha: f64) {
		let start_height = maybe!((start_height));
		self.0.connect_draw(move |window, cr| {
			let allocation = window.allocation();
			cr.set_source_rgba(0.0, 0.0, 0.0, alpha);
			cr.set_operator(cairo::Operator::Screen);

			cr.rectangle(
				0.0,
				0.0 + start_height,
				allocation.width().into(),
				allocation.height() as f64 - start_height,
			);
			let _e = cr.fill();

			false.into()
		});
	}

	pub fn set_pos_inscreen(
		&self,
		monitor: impl AsRef<Monitor>,
		window_width: impl Maybe<i32>,
		window_height: impl Maybe<i32>,
		pos: PosINScreen,
	) {
		let (window_width, window_height) = (
			maybe!((window_width) {window_width} else {self.0.allocated_width()}),
			maybe!((window_height) {window_height} else {self.0.allocated_height()}),
		);
		let (display_width, display_height) = {
			let rect = monitor.as_ref().geometry();
			let w = rect.width();
			let h = rect.height();

			(w, h)
		};
		trace!(
			"#[display] width: {}, height: {}",
			display_width, display_height
		);

		let x = match pos {
			PosINScreen::TopLeft | PosINScreen::CenterLeft | PosINScreen::BottomLeft => 0,
			PosINScreen::TopCenter | PosINScreen::Center | PosINScreen::BottomCenter => {
				(display_width - window_width) / 2
			}
			PosINScreen::TopRight | PosINScreen::RightCenter | PosINScreen::BottomRight => {
				display_width - window_width
			}
		};

		let y = match pos {
			PosINScreen::TopLeft | PosINScreen::TopCenter | PosINScreen::TopRight => 0,
			PosINScreen::CenterLeft | PosINScreen::Center | PosINScreen::RightCenter => {
				(display_height - window_height) / 2
			}
			PosINScreen::BottomLeft | PosINScreen::BottomCenter | PosINScreen::BottomRight => {
				display_height - window_height
			}
		};

		self.0.move_(x, y);
	}

	pub fn adjust_window_height(&self) -> Option<(i32, i32)> {
		let result = (|| {
			let (c_width, c_height) = self.size();
			let new_height = if let Some(child) = self.child() {
				let (m, _) = child.preferred_size();

				m.height
			} else {
				return None;
			};

			if c_height != new_height {
				self.resize(c_width, new_height);

				return Some((c_width, new_height));
			}

			None
		})();
		trace!("adjust_window_height, result: {:?}", result);

		result
	}
}

#[derive(Debug, Deserialize, Clone, Copy, Default)]
pub enum PosINScreen {
	TopLeft = 0,
	CenterLeft = 1,
	BottomLeft = 2,

	TopCenter = 3,
	Center = 4,
	BottomCenter = 5,

	TopRight = 6,
	#[default]
	RightCenter = 7,
	BottomRight = 8,
}

impl PosINScreen {
	pub const fn next(&mut self) -> PosINScreen {
		match self {
			PosINScreen::TopLeft => PosINScreen::CenterLeft,
			PosINScreen::CenterLeft => PosINScreen::BottomLeft,
			PosINScreen::BottomLeft => PosINScreen::TopCenter,
			PosINScreen::TopCenter => PosINScreen::Center,
			PosINScreen::Center => PosINScreen::BottomCenter,
			PosINScreen::BottomCenter => PosINScreen::TopRight,
			PosINScreen::TopRight => PosINScreen::RightCenter,
			PosINScreen::RightCenter => PosINScreen::BottomRight,
			PosINScreen::BottomRight => PosINScreen::TopLeft,
		}
	}
}
