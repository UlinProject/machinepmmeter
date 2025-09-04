use anyhow::Result as anyhowResult;
use anyhow::anyhow;
use clap::Parser;
use directories::ProjectDirs;
use std::path::Path;
use std::path::PathBuf;

use project::APP_PKG_DESCRIPTION;
use project::APP_PKG_NAME;
use project::CONFIG_FILE_NAME;
use project::CONFIG_ORGANIZATION;
use project::CONFIG_QUALIFIER;

#[derive(Parser, Debug)]
#[clap(
	name = APP_PKG_NAME,
	about = APP_PKG_DESCRIPTION
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
				ProjectDirs::from(CONFIG_QUALIFIER, CONFIG_ORGANIZATION, APP_PKG_NAME)
					.ok_or(anyhow!("Could not determine project directories"))
					.map(|a| {
						owned_path = a.config_dir().join(CONFIG_FILE_NAME);

						owned_path.as_path()
					})
			},
			Ok,
		)?;

		next(appconfig_path)
	}
}
