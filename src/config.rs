use anyhow::Result as anyhowResult;
use anyhow::anyhow;
use directories::ProjectDirs;
use gtk::pango;
use serde::Deserialize;
use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;

use crate::Cli;

#[derive(Deserialize, Debug)]
pub struct Config {
	name: Option<String>,
	num_monitor: i32,
	font: FontConfig,
}

#[derive(Deserialize, Debug)]
pub struct FontConfig {
	name: Cow<'static, str>,
	size: f64,
	scale: Option<f64>,
}

impl FontConfig {
	pub fn calc_font_size(&self) -> f64 {
		self.size * self.scale.unwrap_or(pango::SCALE as f64)
	}
}

impl Default for FontConfig {
	fn default() -> Self {
		Self {
			name: "Monospace".into(),
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
			num_monitor: 0,
			font: Default::default(),
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
				ProjectDirs::from("com", "ulinkot", "ryzenpmmeter")
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
	pub fn get_num_monitor(&self) -> i32 {
		self.num_monitor
	}
}
