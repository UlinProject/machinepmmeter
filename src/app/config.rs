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
use crate::core::eight_bitcolor::EightBitColor;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
	name: Option<String>,

	window: WindowAppConfig,
	all_font: FontAppConfig,
	color: ColorAppConfig,
}

#[derive(Deserialize, Debug)]
pub struct WindowAppConfig {
	head: Option<EightBitColor>,
	width: Option<i32>,
	height: Option<i32>,
	transparent: Option<f64>,
	num_monitor: i32,
	pos: PosINScreen,
}

impl Default for WindowAppConfig {
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

impl WindowAppConfig {
	#[inline]
	pub fn get_head_color(&self) -> EightBitColor {
		self.head.unwrap_or_else(|| ColorAppConfig::default().orange())
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
		self.height.map_or(0, |a| a)
	}

	#[inline]
	pub fn get_pos(&self) -> PosINScreen {
		self.pos
	}
}

#[derive(Deserialize, Debug)]
pub struct FontAppConfig {
	family: Cow<'static, str>,
	size: f64,
	scale: Option<f64>,
}

impl FontAppConfig {
	#[inline]
	pub fn calc_font_size(&self) -> f64 {
		self.size * self.scale.unwrap_or(pango::SCALE as f64)
	}

	#[inline]
	pub fn get_family(&self) -> &str {
		&self.family
	}
}

impl Default for FontAppConfig {
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
impl Default for AppConfig {
	fn default() -> Self {
		Self {
			name: None,
			window: WindowAppConfig::default(),
			all_font: FontAppConfig::default(),
			color: ColorAppConfig::default(),
		}
	}
}

#[derive(Deserialize, Debug)]
pub struct ColorAppConfig {
	red: EightBitColor,
	green: EightBitColor,
	orange: EightBitColor,
}

impl ColorAppConfig {
	#[inline]
	pub const fn green(&self) -> EightBitColor {
		self.green
	}

	#[inline]
	pub const fn orange(&self) -> EightBitColor {
		self.orange
	}

	#[inline]
	pub const fn red(&self) -> EightBitColor {
		self.red
	}
}

impl Default for ColorAppConfig {
	fn default() -> Self {
		Self {
			red: EightBitColor::new(255, 51, 51),
			green: EightBitColor::new(153, 255, 51),
			orange: EightBitColor::new(255, 255, 0),
		}
	}
}

impl AppConfig {
	pub fn search_default_path<R>(
		cli: &Cli,
		next: impl FnOnce(&'_ Path) -> anyhowResult<R>,
	) -> anyhowResult<R> {
		let mut owned_path = PathBuf::new();
		let appconfig_path = cli.app_config.as_deref().map_or_else(
			|| {
				ProjectDirs::from("com", "ulinkot", PKG_NAME)
					.ok_or(anyhow!("Could not determine project directories"))
					.map(|a| {
						owned_path = a.config_dir().join("AppConfig.toml");

						owned_path.as_path()
					})
			},
			Ok,
		)?;

		next(appconfig_path)
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
	pub const fn get_window_app_config(&self) -> &WindowAppConfig {
		&self.window
	}

	#[inline]
	pub const fn get_font_app_config(&self) -> &FontAppConfig {
		&self.all_font
	}

	#[inline]
	pub const fn get_color_app_config(&self) -> &ColorAppConfig {
		&self.color
	}
}

impl AsRef<FontAppConfig> for AppConfig {
	#[inline]
	fn as_ref(&self) -> &FontAppConfig {
		self.get_font_app_config()
	}
}

impl AsRef<WindowAppConfig> for AppConfig {
	#[inline]
	fn as_ref(&self) -> &WindowAppConfig {
		self.get_window_app_config()
	}
}

impl AsRef<ColorAppConfig> for AppConfig {
	#[inline]
	fn as_ref(&self) -> &ColorAppConfig {
		self.get_color_app_config()
	}
}
