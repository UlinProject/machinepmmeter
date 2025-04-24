use gtk::gdk::{self, Visual, traits::MonitorExt};

#[derive(Debug)]
pub struct ViGraphDisplayInfo {
	#[allow(dead_code)]
	display: gdk::Display,
	screen: gdk::Screen,
	monitor: gdk::Monitor,
}

impl AsRef<gdk::Screen> for ViGraphDisplayInfo {
	#[inline]
	fn as_ref(&self) -> &gdk::Screen {
		&self.screen
	}
}

impl AsRef<gdk::Display> for ViGraphDisplayInfo {
	#[inline]
	fn as_ref(&self) -> &gdk::Display {
		&self.display
	}
}

impl AsRef<gdk::Monitor> for ViGraphDisplayInfo {
	#[inline]
	fn as_ref(&self) -> &gdk::Monitor {
		&self.monitor
	}
}

impl ViGraphDisplayInfo {
	pub fn new(num_monitor: i32) -> Option<Self> {
		let display = gdk::Display::default()?;
		let monitor = display.monitor(num_monitor)?;
		let screen = display.default_screen();

		Self {
			display,
			screen,
			monitor,
		}
		.into()
	}

	pub fn monitor_width_and_height(&self) -> (i32, i32) {
		let rect = self.monitor.geometry();
		let w = rect.width();
		let h = rect.height();

		(w, h)
	}

	#[allow(dead_code)]
	#[inline]
	pub fn screen_rgba_visual(&self) -> Option<Visual> {
		self.screen.rgba_visual()
	}
}
