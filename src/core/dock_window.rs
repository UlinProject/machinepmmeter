use std::ops::Deref;

use crate::{__gen_transparent_gtk_type, core::display::ViGraphDisplayInfo};
use gtk::{
	Application, ApplicationWindow,
	ffi::GtkApplicationWindow,
	gdk::{Screen, WindowTypeHint},
	traits::{GtkWindowExt, WidgetExt},
};

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

	pub fn set_pos_inscreen(&self, vi: &ViGraphDisplayInfo, pos: PosINScreen) {
		let (window_width, window_height) = (self.0.allocated_width(), self.0.allocated_height());
		let (display_width, display_height) = vi.monitor_width_and_height();

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

#[derive(Debug, Clone, Copy, Default)]
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
