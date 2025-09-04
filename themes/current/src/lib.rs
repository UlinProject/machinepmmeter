// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Denis Kotlyarov (Денис Котляров) <denis2005991@gmail.com>

impl_current_theme!(name_theme: "udefstyle");

macro_rules! impl_current_theme {
	[
		name_theme: $name:expr
	] => {
		$crate::inject! {
			$crate::parse_and_impl_theme_toml! {
				#tt("themes/" $name "/theme.toml")
			}
		}
	};
}

mod parse_theme_toml;

pub(crate) use impl_current_theme;
pub(crate) use include_tt::inject;
pub(crate) use parse_theme_toml::parse_and_impl_theme_toml;
