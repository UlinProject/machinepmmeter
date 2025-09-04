// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Denis Kotlyarov (Денис Котляров) <denis2005991@gmail.com>

use crate::app::cli::AppCli;
use crate::app::config::AppConfig;
use project::{APP_ID, UPPERCASE_APP_PKG_VERSION};
use crate::app::keyboard::spawn_keyboard_thread;
use crate::app::traymenu::app_traymenu;
use crate::app::windows::main::AppMain;
use crate::core::display::ViGraphDisplayInfo;
use anyhow::{Context, Result as anyhowResult, bail};
use clap::Parser;
use log::{info, trace};
use maybe::maybetype;
use std::io::{Write, stderr};
use std::rc::Rc;
use std::{fs, panic};

mod widgets;
mod core {
	pub mod display;
	pub mod eightbitcolor;
	pub mod gtkcodegen;
	pub mod traymenu;
}

pub mod app {
	pub mod windows {
		pub mod aboutdialog;
		pub mod main;
	}
	pub mod cli;
	pub mod config;
	pub mod dockwindow;
	pub mod events;
	pub mod keyboard;
	pub mod traymenu;
}

pub mod metrics {
	#[cfg(feature = "demo_mode")]
	#[cfg_attr(docsrs, doc(cfg(feature = "demo_mode")))]
	pub mod demo;
	pub mod lm_sensors;
	pub mod sysinfo;
	pub mod udisks2;
}

maybetype!(
	Maybe: (i32, f64, gtk::gdk::RGBA, &'_ str, String, usize, gtk::pango::Weight);
);

#[global_allocator]
pub static GLOBAL: allocators::Allocator = allocators::GLOBAL;

fn main() -> anyhowResult<()> {
	println!("{APP_ID}:");
	panic::set_hook(Box::new(|p_hook_info| {
		{
			let stderr = stderr();
			let mut lock = stderr.lock();

			let _e = writeln!(
				lock,
				"###\n## The application cannot continue its operation due to a panic detected:\n###\n{p_hook_info}"
			);
			let _e = lock.flush();
		}

		std::process::exit(-1);
	}));

	#[cfg(feature = "no-gui-root")]
	#[cfg_attr(docsrs, doc(cfg(feature = "no-gui-root")))]
	if unsafe { libc::getuid() == 0 } {
		bail!("Do not run graphical applications with root user rights.");
	}

	env_logger::try_init()?;
	let cli = AppCli::parse();

	let app_config = cli.search_default_appconfigpath(|app_config_path| {
		let allow_save_default_app_config = cli.get_allow_save_default_app_config();
		info!(
			"#[AppConfig file] open: {app_config_path:?}, allow_save_default_AppConfig: {allow_save_default_app_config:?}"
		);
		let app_config = {
			let context = || format!("Open AppConfig file {:?}.", cli.get_app_config());
			let app_config = fs::read_to_string(app_config_path).map_or_else(
				|e| match allow_save_default_app_config {
					false => Err(e).with_context(context),
					true => {
						let app_config = AppConfig::default();

						Ok(app_config)
					}
				},
				|rdata| toml::from_str(&rdata).with_context(context),
			);

			Rc::new(app_config?)
		};

		Ok(app_config)
	})?;
	trace!("#[AppConfig file] current: {app_config:?}");

	gtk::init()?;
	// Display
	let display = Rc::new(ViGraphDisplayInfo::new(
		app_config.get_window_app_config().get_num_monitor(),
	)?);
	// AppEvents
	let (tx_appevents, rx_appevents) = crate::app::events::app_events_channel();
	let rx_appevents = Rc::new(rx_appevents);
	// App
	let app = AppMain::new(
		APP_ID,
		UPPERCASE_APP_PKG_VERSION,
		current_theme::A_THEME_MAIN_CSS,
		app_config,
		&display,
		rx_appevents,
	)?;
	// TrayMenu
	let app_traymenu = app_traymenu(&tx_appevents);
	//

	// Keyboard
	spawn_keyboard_thread(tx_appevents);

	// Run
	app.run();
	drop(app_traymenu);

	Ok(())
}
