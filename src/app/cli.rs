use crate::PKG_DESCRIPTION;
use crate::PKG_NAME;
use anyhow::Result as anyhowResult;
use anyhow::anyhow;
use clap::Parser;
use directories::ProjectDirs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(
	name = PKG_NAME,
	about = PKG_DESCRIPTION
)]
pub struct AppCli {
	/// Path to the TOML AppConfiguration file
	#[clap(short, long, value_parser, default_value = None)]
	app_config: Option<PathBuf>,

	/// Allow saving default AppConfig if it doesn't exist
	#[clap(long, value_parser, default_value = "true")]
	allow_save_default_app_config: bool,
}

impl AppCli {
	#[inline]
	pub fn get_app_config(&self) -> Option<&Path> {
		self.app_config.as_deref()
	}

	#[inline]
	pub fn get_allow_save_default_app_config(&self) -> bool {
		self.allow_save_default_app_config
	}

	pub fn search_default_appconfigpath<R>(
		&self,
		next: impl FnOnce(&'_ Path) -> anyhowResult<R>,
	) -> anyhowResult<R> {
		let mut owned_path = PathBuf::new();
		let appconfig_path = self.get_app_config().map_or_else(
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
}
