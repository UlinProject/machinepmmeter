use anyhow::Result as anyhowResult;
use anyhow::anyhow;
use directories::ProjectDirs;
use gtk::pango;
use serde::Deserialize;
use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;

use crate::Cli;
use crate::PKG_NAME;
use crate::UPPERCASE_PKG_NAME;
use crate::core::dock_window::PosINScreen;

#[derive(Deserialize, Debug)]
pub struct Config {
	name: Option<String>,

	window: WindowConfig,
	font: FontConfig,
	color: ColorConfig,
}

#[derive(Deserialize, Debug)]
pub struct WindowConfig {
	head: Option<(u8, u8, u8)>,
	width: Option<i32>,
	height: Option<i32>,
	transparent: Option<f64>,
	num_monitor: i32,
	pos: PosINScreen,
}

impl Default for WindowConfig {
	#[inline]
	fn default() -> Self {
		Self {
			head: None,
			width: None,
			height: None,
			transparent: 0.5.into(),
			num_monitor: 0,
			pos: Default::default(),
		}
	}
}

impl WindowConfig {
	#[inline]
	pub fn get_head_color(&self) -> (u8, u8, u8) {
		self.head.unwrap_or_else(|| ColorConfig::default().orange())
	}

	#[inline]
	pub const fn get_transparent(&self) -> Option<f64> {
		self.transparent
	}

	#[inline]
	pub const fn get_num_monitor(&self) -> i32 {
		self.num_monitor
	}

	pub fn get_width_or_default(&self) -> i32 {
		self.width.map_or(240, |a| a)
	}

	pub fn get_height_or_default(&self) -> i32 {
		self.height.map_or(50, |a| a)
	}

	#[inline]
	pub fn get_pos(&self) -> PosINScreen {
		self.pos
	}
}

#[derive(Deserialize, Debug)]
pub struct FontConfig {
	family: Cow<'static, str>,
	size: f64,
	scale: Option<f64>,
}

impl FontConfig {
	#[inline]
	pub fn calc_font_size(&self) -> f64 {
		self.size * self.scale.unwrap_or(pango::SCALE as f64)
	}

	#[inline]
	pub fn get_family(&self) -> &str {
		&self.family
	}
}

impl Default for FontConfig {
	fn default() -> Self {
		Self {
			//family: "Monospace".into(),
			family: "Inter,sans-serif".into(),
			size: 12.0,
			scale: Default::default(),
		}
	}
}

#[allow(clippy::derivable_impls)]
impl Default for Config {
	fn default() -> Self {
		Self {
			name: None,
			window: WindowConfig::default(),
			font: FontConfig::default(),
			color: ColorConfig::default(),
		}
	}
}

#[derive(Deserialize, Debug)]
pub struct ColorConfig {
	red: (u8, u8, u8),
	green: (u8, u8, u8),
	orange: (u8, u8, u8),
}

impl ColorConfig {
	#[inline]
	pub const fn green(&self) -> (u8, u8, u8) {
		self.green
	}

	#[inline]
	pub const fn orange(&self) -> (u8, u8, u8) {
		self.orange
	}

	#[inline]
	pub const fn red(&self) -> (u8, u8, u8) {
		self.red
	}
}

impl Default for ColorConfig {
	fn default() -> Self {
		Self {
			red: (255, 51, 51),
			green: (153, 255, 51),
			orange: (255, 255, 0),
		}
	}
}

impl Config {
	pub fn search_default_path<R>(
		cli: &Cli,
		next: impl FnOnce(&'_ Path) -> anyhowResult<R>,
	) -> anyhowResult<R> {
		let mut owned_path = PathBuf::new();
		let config_path = cli.config.as_deref().map_or_else(
			|| {
				ProjectDirs::from("com", "ulinkot", PKG_NAME)
					.ok_or(anyhow!("Could not determine project directories"))
					.map(|a| {
						owned_path = a.config_dir().join("config.toml");

						owned_path.as_path()
					})
			},
			Ok,
		)?;

		next(config_path)
	}

	#[inline]
	pub fn get_name(&self) -> Option<&str> {
		self.name.as_deref()
	}

	#[inline]
	pub fn get_name_or_default(&self) -> &str {
		self.get_name().unwrap_or(UPPERCASE_PKG_NAME)
	}

	#[inline]
	pub const fn get_window_config(&self) -> &WindowConfig {
		&self.window
	}

	#[inline]
	pub const fn get_font_config(&self) -> &FontConfig {
		&self.font
	}

	#[inline]
	pub const fn get_color_config(&self) -> &ColorConfig {
		&self.color
	}
}

impl AsRef<FontConfig> for Config {
	#[inline]
	fn as_ref(&self) -> &FontConfig {
		self.get_font_config()
	}
}

impl AsRef<WindowConfig> for Config {
	#[inline]
	fn as_ref(&self) -> &WindowConfig {
		self.get_window_config()
	}
}

impl AsRef<ColorConfig> for Config {
	#[inline]
	fn as_ref(&self) -> &ColorConfig {
		self.get_color_config()
	}
}
