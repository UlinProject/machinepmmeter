// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Denis Kotlyarov (Денис Котляров) <denis2005991@gmail.com>

impl_current_project!(name_conf: "project");

macro_rules! impl_current_project {
	[
		name_conf: $name:expr
	] => {
		$crate::include_tt! {
			$crate::no_comments_toml! {
				@in [ #include_tt!([$name ".toml"]) ]
				@result []
				@end [ $crate::parse_and_impl_project_toml ]
			}
		}
	};
}

mod parse_project_toml;
mod no_comments_toml;

#[allow(unused_imports)]
pub(crate) use cluConstData::concat_str;
#[allow(unused_imports)]
pub(crate) use constuppercase::const_ascii_uppercase;
#[allow(unused_imports)]
pub(crate) use impl_current_project;
#[allow(unused_imports)]
pub(crate) use include_tt::include_tt;
#[allow(unused_imports)]
pub(crate) use no_comments_toml::no_comments_toml;
#[allow(unused_imports)]
pub(crate) use parse_project_toml::parse_and_impl_project_toml;

pub mod default_project_toml {
	// config
	pub const CONFIG_QUALIFIER: &str = "com";
	pub const CONFIG_ORGANIZATION: &str = "ulinkot";
	pub const CONFIG_FILE_NAME: &str = "AppConfig.toml";

	// app
	pub const APP_ICON: &str = env!("CARGO_PKG_NAME");
	pub const APP_WEBSITE: &str = env!("CARGO_PKG_REPOSITORY");
	pub const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
	pub const APP_COPYRIGHT: &str = "© 2025 Denis Kotlyarov";
	pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
	pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
	pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
}
