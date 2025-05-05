// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Denis Kotlyarov (Денис Котляров) <denis2005991@gmail.com>

use crate::app::aboutdialog::AppAboutDialog;
use crate::app::cli::AppCli;
use crate::app::config::AppConfig;
use crate::app::dockwindow::{AppViDockWindow, PosINScreen};
use crate::app::events::{AppEventSender, AppEvents};
use crate::app::keyboard::{AppKeyboardEvents, spawn_keyboard_thread};
use crate::app::traymenu::app_traymenu;
use crate::core::display::ViGraphDisplayInfo;
use crate::widgets::dockhead::ViDockHead;
use crate::widgets::hotkeys::ViHotkeyItems;
use crate::widgets::notebook::ViNotebook;
use crate::widgets::primitives::graph::ViGraphBackgroundSurface;
use anyhow::{Context, Result as anyhowResult};
use async_channel::Receiver;
use clap::Parser;
use enclose::enc;
use gtk::gdk::{Monitor, Screen};
use gtk::gio::prelude::ApplicationExtManual;
use gtk::gio::traits::ApplicationExt;
use gtk::glib::Cast;
use gtk::prelude::{NotebookExtManual, WidgetExt};
use gtk::traits::{
	BinExt, BoxExt, ContainerExt, CssProviderExt, GtkWindowExt, NotebookExt, ScrolledWindowExt,
};
use gtk::{Application, ScrolledWindow};
use gtk::{Box as GtkBox, CssProvider};
use log::{info, trace, warn};
use std::cell::RefCell;
use std::io::{Write, stderr};
use std::num::NonZero;
use std::rc::Rc;
use std::time::Duration;
use std::{fs, panic};

mod widgets;
mod core {
	pub mod constuppercase;
	pub mod display;
	pub mod eightbitcolor;
	pub mod gtkcodegen;
	pub mod keyboard;
	pub mod maybe;
	pub mod traymenu;
}

pub mod app {
	pub mod aboutdialog;
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
}

const APP_ID: &str = "com.ulinkot.machinepmmeter";
const PKG_ICON: &str = env!("CARGO_PKG_NAME");
const PKG_WEBSITE: &str = env!("CARGO_PKG_REPOSITORY");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PKG_COPYRIGHT: &str = "© 2025 Denis Kotlyarov";
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const UPPERCASE_PKG_NAME: &str = const_ascii_uppercase!(PKG_NAME);

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const UPPERCASE_PKG_VERSION: &str = const_ascii_uppercase!(PKG_VERSION);

const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> anyhowResult<()> {
	panic::set_hook(Box::new(|p_hook_info| {
		{
			let stderr = stderr();
			let mut lock = stderr.lock();

			let _e = writeln!(
				lock,
				"###\n## The application cannot continue its operation due to a panic detected:\n###\n{}",
				p_hook_info
			);
			let _e = lock.flush();
		}

		std::process::exit(-1);
	}));

	if unsafe { libc::getuid() == 0 } {
		panic!("Do not run graphical applications with root user rights.");
	}

	env_logger::try_init()?;
	let cli = AppCli::parse();

	let app_config = cli.search_default_appconfigpath(|app_config_path| {
		let allow_save_default_app_config = cli.get_allow_save_default_app_config();
		info!(
			"#[AppConfig file] open: {:?}, allow_save_default_AppConfig: {:?}",
			app_config_path, allow_save_default_app_config
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
	trace!("#[AppConfig file] current: {:?}", app_config);

	gtk::init()?;
	let c_display = Rc::new(ViGraphDisplayInfo::new(
		app_config.get_window_app_config().get_num_monitor(),
	)?);
	let defcss = {
		let a_css = CssProvider::new();
		a_css.load_from_data(include_bytes!("../style/def.css"))?;

		a_css
	};

	let (tx_appevents, rx_appevents) = crate::app::events::app_event_channel();
	let rx_appevents = Rc::new(rx_appevents);
	let app_traymenu = app_traymenu(&tx_appevents);

	let application = Application::new(Some(APP_ID), Default::default());
	application.connect_activate(enc!((app_config, rx_appevents) move |app| {
		gtk::StyleContext::add_provider_for_screen(
			AsRef::<Screen>::as_ref(&c_display as &ViGraphDisplayInfo),
			&defcss,
			gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
		);

		let name_window = app_config.get_name_or_default();
		build_ui(app, name_window, &app_config, &c_display, tx_appevents.clone(), rx_appevents.clone());
	}));

	application.run();
	drop(app_traymenu);
	Ok(())
}

#[allow(clippy::too_many_arguments)]
fn build_ui(
	app: &gtk::Application,
	name_window: &str,
	app_config: &Rc<AppConfig>,
	c_display: &Rc<ViGraphDisplayInfo>,

	esender: AppEventSender,
	receiver: Rc<Receiver<AppEvents>>,
) {
	trace!("#[gui] Start initialization, name: {:?}", name_window);

	let dock_window = AppViDockWindow::new(app, name_window, &**app_config);
	let pos_inscreen = Rc::new(RefCell::new(app_config.get_window_app_config().get_pos()));
	let vigraph_surface = ViGraphBackgroundSurface::default();
	let vbox = GtkBox::new(gtk::Orientation::Vertical, 0);
	vbox.set_valign(gtk::Align::Fill);
	vbox.set_halign(gtk::Align::Baseline);

	let vinotebook = ViNotebook::new(c_display, &dock_window, &pos_inscreen);
	#[cfg(feature = "demo_mode")]
	#[cfg_attr(docsrs, doc(cfg(feature = "demo_mode")))]
	{
		crate::metrics::demo::vinotebook_append_page(
			app_config,
			&vigraph_surface,
			dock_window.allocation().width(),
			200,
			&vinotebook,
		);
	}

	{
		crate::metrics::lm_sensors::vinotebook_append_page(
			app_config,
			&vigraph_surface,
			dock_window.allocation().width(),
			2400,
			Duration::from_millis(12),
			unsafe { NonZero::new_unchecked(10) },
			Duration::from_millis(1),
			&vinotebook,
		);
	}

	if let Some(level) = app_config.get_window_app_config().get_transparent() {
		if level != 1.0 {
			match c_display.screen_rgba_visual() {
				Some(visual) => {
					dock_window.set_visual(Some(&visual));

					let mut is_connected_notepadlabelrealized = false;
					if let Some(child) = vinotebook.nth_page(Some(0)) {
						if let Some(tab_label) = vinotebook.tab_label(&child) {
							tab_label.connect_realize(enc!((dock_window) move |tab_label| {
								let notebook_labelheight = tab_label.allocated_height();

								dock_window.connect_transparent_background(notebook_labelheight as f64, level);
							}));
							is_connected_notepadlabelrealized = true;
						}
					}

					if !is_connected_notepadlabelrealized {
						dock_window.connect_transparent_background(30.0, level);
					}
				}
				None => {
					warn!("#[gui] Transparency was expected, but the system does not support it");
				}
			}
		}
	}

	vbox.pack_start(&vinotebook, true, true, 0);
	vbox.pack_end(
		&ViDockHead::new(app_config.clone(), name_window, UPPERCASE_PKG_VERSION, 1.0),
		true,
		true,
		0,
	);

	dock_window.set_child(Some(&vbox));
	vbox.set_visible(true);

	spawn_keyboard_thread(esender);

	dock_window.connect_show(
		enc!((pos_inscreen, c_display, dock_window, vinotebook) move |_| {
			trace!("connect_show: ");
			if let Some(c_page) = vinotebook.current_page() {
				if let Some(child) = vinotebook.nth_page(Some(c_page)) {
					if let Ok(scrolled_window) = child.downcast::<ScrolledWindow>() {
						let height = if let Some(child) = scrolled_window.child() {
							let (m, _) = child.preferred_size();
							m.height
						} else {
							0
						};

						scrolled_window.set_hexpand(true);
						scrolled_window.set_vexpand(true);
						scrolled_window.set_size_request(-1, height);

						scrolled_window.set_max_content_height(i32::MAX);
					}
				}
			}

			if let Some((window_width, height_window)) = dock_window.adjust_window_height() {
				dock_window.set_pos_inscreen(&*c_display, window_width, height_window, *pos_inscreen.borrow());
			}
		}),
	);

	dock_window.connect_resize_mode_notify(enc!((pos_inscreen, c_display, dock_window) move |_| {
		trace!("connect_resize_mode_notify: ");

		dock_window.set_pos_inscreen(&*c_display, (), (), *pos_inscreen.borrow());
	}));
	dock_window.connect_screen_changed(
		enc!((pos_inscreen, app_config, c_display) move |dock_window, screen| {
			trace!("connect_screen_changed: ");
			let pos_inscreen: PosINScreen = *pos_inscreen.borrow();
			let mut owned_motitor = None;
			let c_monitor: &Monitor = ViGraphDisplayInfo::as_ref(&*c_display);
			let monitor: &Monitor = screen
				.map(|a| a.display())
				.and_then(|a| {
					owned_motitor = a.monitor(app_config.get_window_app_config().get_num_monitor());
					owned_motitor.as_ref()
			}).unwrap_or(c_monitor);

			dock_window.set_pos_inscreen(monitor, (), (), pos_inscreen);
		}),
	);

	glib::MainContext::default().spawn_local(
		enc!((c_display, dock_window, pos_inscreen, vbox, app_config) async move {
			trace!("main_thread: ");
			let app_about_dialog = Rc::new(RefCell::new(None));
			let mut wdock_vihotkey = None;
			while let Ok(event) = receiver.recv().await {
				match event {
					AppEvents::Keyboard(AppKeyboardEvents::Num1) => vinotebook.set_page(0),
					AppEvents::Keyboard(AppKeyboardEvents::Num2) => vinotebook.set_page(1),
					AppEvents::Keyboard(AppKeyboardEvents::Num3) => vinotebook.set_page(2),
					AppEvents::Keyboard(AppKeyboardEvents::Num4) => vinotebook.set_page(3),
					AppEvents::Keyboard(AppKeyboardEvents::Num5) => vinotebook.set_page(4),
					AppEvents::Keyboard(AppKeyboardEvents::Num6) => vinotebook.set_page(5),
					AppEvents::Keyboard(AppKeyboardEvents::Num7) => vinotebook.set_page(6),
					AppEvents::Keyboard(AppKeyboardEvents::Num8) => vinotebook.set_page(7),
					AppEvents::Keyboard(AppKeyboardEvents::Num9) => vinotebook.set_page(8),
					AppEvents::MoveTabToPrevPosition | AppEvents::Keyboard(AppKeyboardEvents::KeyA) => {
						let mut a_page = vinotebook.current_page().unwrap_or(1);
						if a_page == 0 {
							a_page = vinotebook.n_pages() - 1;
						}else {
							a_page -= 1;
						}

						vinotebook.set_page(a_page as _);
					},
					AppEvents::MoveTabToNextPosition | AppEvents::Keyboard(AppKeyboardEvents::KeyD) => {
						let mut a_page = vinotebook.current_page().unwrap_or(0) + 1;
						if a_page >= vinotebook.n_pages() {
							a_page = 0;
						}

						vinotebook.set_page(a_page as _);
					},
					AppEvents::ToggleDockWindowVisibility | AppEvents::Keyboard(AppKeyboardEvents::ShiftF8) if dock_window.is_visible() => {
						dock_window.hide();
					},
					AppEvents::ToggleDockWindowVisibility | AppEvents::Keyboard(AppKeyboardEvents::ShiftF8) => {
						dock_window.show();
					},
					AppEvents::Exit | AppEvents::Keyboard(AppKeyboardEvents::Escape) => {
						dock_window.close();
						gtk::main_quit();
					},
					AppEvents::ShowOrFocusAboutDialog => {
						let mut write_aad = RefCell::borrow_mut(&app_about_dialog);
						match *write_aad {
							None => {
								let aad = AppAboutDialog::new(enc!((app_about_dialog) move || {
									*RefCell::borrow_mut(&app_about_dialog) = None;
								}));

								aad.show_all();
								aad.present();

								*write_aad = Some(aad);
							},
							Some(ref a) => a.present(),
						}
					},
					AppEvents::Keyboard(AppKeyboardEvents::KeyPlus) => {},
					AppEvents::Keyboard(AppKeyboardEvents::KeyMinus) => {},
					AppEvents::MoveDockWindowToNextPosition | AppEvents::Keyboard(AppKeyboardEvents::KeyP) => {
						let new_pos = { // NEXT POS IN SCREEN
							let mut write = pos_inscreen.borrow_mut();
							let new_pos = write.next();
							*write = new_pos;

							new_pos
						};
						dock_window.set_pos_inscreen(&*c_display, (), (), new_pos);
					},
					AppEvents::KeyboardListenerEnabled(true) => {
						if wdock_vihotkey.is_none() {
							let arr = [
								("view-conceal-symbolic", "Hide | Show", "(Shift and F8)"),
								("zoom-original-symbolic", "Selecting a tab", "(Shift and 1 | 2 | ..)"),
								("go-next-symbolic", "Next tab", "(Shift and D)"),
								("go-previous-symbolic", "Previous tab", "(Shift and A)"),
								(
									"sidebar-show-right-symbolic-rtl",
									"Next position", "(Shift and P)",
								),
								("system-shutdown-symbolic", "Exit", "(Shift and Esc)")
							];
							let vihotkey = ViHotkeyItems::new(
								&*app_config,
								"# Hot keys",
								arr.into_iter(),
								1.0,
							);
							vbox.add(&vihotkey);
							vihotkey.set_visible(true);

							wdock_vihotkey = Some(vihotkey);
						}
					},
					AppEvents::KeyboardListenerEnabled(false) => {
						if let Some(vihotkey) = wdock_vihotkey {
							vihotkey.set_visible(false);
							vbox.remove(&vihotkey);

							wdock_vihotkey = None;

							if let Some((window_width, height_window)) = dock_window.adjust_window_height() {
								dock_window.set_pos_inscreen(&*c_display, window_width, height_window, *pos_inscreen.borrow());
							}
						}
					},
				}
			}
		}),
	);

	glib::MainContext::default().spawn_local(enc!((dock_window) async move {
		dock_window.present();
	}));
}
