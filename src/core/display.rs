use anyhow::Result as anyhowResult;
use anyhow::anyhow;
use gtk::gdk::{self, Visual};

#[derive(Debug)]
pub struct ViGraphDisplayInfo {
	screen: gdk::Screen,
	monitor: gdk::Monitor,
}

impl AsRef<gdk::Screen> for ViGraphDisplayInfo {
	#[inline]
	fn as_ref(&self) -> &gdk::Screen {
		&self.screen
	}
}

impl AsRef<gdk::Monitor> for ViGraphDisplayInfo {
	#[inline]
	fn as_ref(&self) -> &gdk::Monitor {
		&self.monitor
	}
}

impl ViGraphDisplayInfo {
	pub fn new(num_monitor: i32) -> anyhowResult<Self> {
		let display = gdk::Display::default()
			.ok_or_else(|| anyhow!("Failed to read display characteristics"))?;
		let monitor = display.monitor(num_monitor).ok_or_else(|| {
			anyhow!(
				"Failed to read monitor characteristics, args: num_monitor={}",
				num_monitor
			)
		})?;
		let screen = display.default_screen();

		Ok(Self { screen, monitor })
	}

	#[allow(dead_code)]
	#[inline]
	pub fn screen_rgba_visual(&self) -> Option<Visual> {
		self.screen.rgba_visual()
	}
}
