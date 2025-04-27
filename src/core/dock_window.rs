use crate::{__gen_transparent_gtk_type, config::WindowConfig};
use gtk::{
	Application, ApplicationWindow, cairo,
	ffi::GtkApplicationWindow,
	gdk::{Monitor, Screen, WindowTypeHint, traits::MonitorExt},
	traits::{GtkWindowExt, WidgetExt},
};
use log::trace;
use serde::Deserialize;
use std::ops::Deref;

#[repr(transparent)]
#[derive(Debug)]
pub struct ViDockWindow(ApplicationWindow);

__gen_transparent_gtk_type! {
	#[sys(GtkApplicationWindow)]
	ViDockWindow(
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

impl Deref for ViDockWindow {
	type Target = ApplicationWindow;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ViDockWindow {
	pub fn new(app: &'_ Application, title: &'_ str, wconfig: impl AsRef<WindowConfig>) -> Self {
		let wconfig = wconfig.as_ref();
		let window = ApplicationWindow::new(app);
		window.set_visible(false);
		window.set_title(title);
		window.set_decorated(false);
		window.set_app_paintable(true);
		window.set_default_size(
			wconfig.get_width_or_default(),
			wconfig.get_height_or_default(),
		);
		window.set_keep_above(true);
		window.set_type_hint(WindowTypeHint::Dock);

		Self(window)
	}

	pub fn connect_transparent_background(
		&self,
		screen: impl AsRef<Screen>,
		alpha: f64,
	) -> HasTransparent {
		fn __connect_transparent_background(
			app_window: &ApplicationWindow,
			screen: &Screen,
			alpha: f64,
		) -> HasTransparent {
			if let Some(visual) = screen.rgba_visual() {
				app_window.set_visual(Some(&visual));

				app_window.connect_draw(move |window, cr| {
					let allocation = window.allocation();
					cr.set_source_rgba(0.0, 0.0, 0.0, alpha);
					cr.set_operator(cairo::Operator::Screen);

					cr.rectangle(
						0.0,
						0.0,
						allocation.width().into(),
						allocation.height().into(),
					);
					let _e = cr.fill();

					false.into()
				});

				HasTransparent::True
			} else {
				HasTransparent::False
			}
		}

		__connect_transparent_background(&self.0, screen.as_ref(), alpha)
	}

	pub fn set_pos_inscreen(&self, monitor: impl AsRef<Monitor>, pos: PosINScreen) {
		let (window_width, window_height) = (self.0.allocated_width(), self.0.allocated_height());
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
}

#[derive(Debug, Deserialize, Clone, Copy, Default)]
pub enum PosINScreen {
	TopLeft = 0,
	CenterLeft = 1,
	BottomLeft = 2,

	TopCenter = 3,
	Center = 4,
	BottomCenter = 5,

	#[default]
	TopRight = 6,
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

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum HasTransparent {
	True = true as _,
	False = false as _,
}

impl HasTransparent {
	#[inline]
	pub const fn is_true(&self) -> bool {
		matches!(self, Self::True)
	}

	#[inline]
	pub const fn unwrap_or(&self, default: bool) -> bool {
		match self {
			Self::False => default,
			Self::True => true,
		}
	}
}
