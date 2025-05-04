// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Denis Kotlyarov (Денис Котляров) <denis2005991@gmail.com>

use crate::app::aboutdialog::AppAboutDialog;
use crate::app::cli::AppCli;
use crate::app::config::AppConfig;
use crate::app::dockwindow::{AppViDockWindow, PosINScreen};
use crate::app::events::{AppEventSender, AppEvents};
use crate::app::keyboard::{AppKeyboardEvents, spawn_keyboard_thread};
use crate::app::traymenu::{AppTrayMenu, AppTrayMenuItem};
use crate::core::display::ViGraphDisplayInfo;
use crate::widgets::ViMeter;
use crate::widgets::dockhead::ViDockHead;
use crate::widgets::hotkeys::ViHotkeyItems;
use crate::widgets::primitives::graph::ViGraphBackgroundSurface;
use crate::widgets::primitives::iconmenuitem::ViIconMenuItem;
use crate::widgets::primitives::label::ViLabel;
use anyhow::anyhow;
use anyhow::{Context, Result as anyhowResult};
use async_channel::Receiver;
use clap::Parser;
use enclose::enc;
use glib::ControlFlow;
use gtk::gdk::{Monitor, Screen};
use gtk::gio::prelude::ApplicationExtManual;
use gtk::gio::traits::ApplicationExt;
use gtk::glib::Cast;
use gtk::pango::{Weight, WrapMode};
use gtk::prelude::{NotebookExtManual, WidgetExt};
use gtk::traits::{
	BinExt, BoxExt, ContainerExt, CssProviderExt, GtkMenuItemExt, GtkWindowExt, NotebookExt,
	ScrolledWindowExt, StyleContextExt,
};
use gtk::{Align, Application, Notebook, ScrolledWindow};
use gtk::{Box as GtkBox, CssProvider};
use lm_sensors::{LMSensors, SubFeatureRef};
use log::{info, trace, warn};
use std::cell::RefCell;
use std::io::{Write, stderr};
use std::rc::Rc;
use std::{fs, panic};

mod widgets;
mod core {
	pub mod constuppercase;
	pub mod display;
	pub mod eightbitcolor;
	pub mod gtkcodegen;
	pub mod keyboard;
	pub mod maybe;
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
	let tray_menu = {
		// Tray menu
		let hide_or_show = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
			vi.connect_activate(enc!((tx_appevents) move |_| {
				tx_appevents.toggle_window_visibility();
			}));
		}) as &'_ mut dyn FnMut(&'_ mut ViIconMenuItem);

		let next_position = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
			vi.connect_activate(enc!((tx_appevents) move |_| {
				tx_appevents.move_window_to_next_position();
			}));
		});

		let abouttheprogram = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
			vi.connect_activate(enc!((tx_appevents) move |_| {
				tx_appevents.show_or_focus_aboutdialog();
			}));
		});

		let exit = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
			vi.connect_activate(enc!((tx_appevents) move |_| {
				tx_appevents.exit();
			}));
		});

		let tray_menu = AppTrayMenu::new(
			APP_ID,
			PKG_ICON,
			PKG_NAME,
			PKG_DESCRIPTION,
			[
				AppTrayMenuItem::icon_item("view-conceal-symbolic", "Hide | Show", hide_or_show),
				AppTrayMenuItem::icon_item(
					"sidebar-show-right-symbolic-rtl",
					"Next position",
					next_position,
				),
				AppTrayMenuItem::Separator,
				AppTrayMenuItem::item("About the program", abouttheprogram),
				AppTrayMenuItem::icon_item("system-shutdown-symbolic", "Exit", exit),
			]
			.into_iter(),
		);

		tray_menu
	};

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
	drop(tray_menu);
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
	let (c_transparent, _is_transparent_mode) = app_config
		.get_window_app_config()
		.get_transparent()
		.map_or_else(
			|| (1.0, false),
			|level| match !dock_window
				.connect_transparent_background(&**c_display, level)
				.is_true()
			{
				false => {
					warn!("#[gui] Transparency was expected, but the system does not support it");

					(1.0, false)
				}
				true => (level, true),
			},
		);

	let pos_inscreen = Rc::new(RefCell::new(app_config.get_window_app_config().get_pos()));
	let vigraph_surface = ViGraphBackgroundSurface::default();
	let vbox = GtkBox::new(gtk::Orientation::Vertical, 0);
	vbox.set_valign(gtk::Align::Fill);
	vbox.set_halign(gtk::Align::Baseline);

	let notebook = {
		let notebook = Notebook::new();
		notebook.style_context().add_class("vinotebook");
		notebook.connect_switch_page(
			enc!((dock_window, c_display, pos_inscreen) move |notebook, _page, page_num| {
				for i in 0..notebook.n_pages() {
					if let Some(child) = notebook.nth_page(Some(i)) {
						if let Some(tab_label) = notebook.tab_label(&child) {
							if let Some(label) = tab_label.downcast_ref::<ViLabel>() {
								let style = label.style_context();

								if i != 0 && !style.has_class("notfirst_head_vinotebook") {
									style.add_class("notfirst_head_vinotebook");
								}

								if i == page_num {
									if !style.has_class("active_head_vinotebook") {
										style.add_class("active_head_vinotebook");

										label.read_text(|text| {
											label.set_text(&format!("# {}", text));
										});

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
									continue;
								}

								if style.has_class("active_head_vinotebook") {
									style.remove_class("active_head_vinotebook");

									label.read_text(|text| {
										if let Some(next_text) = text.strip_prefix("# ") {
											label.set_text(next_text);
										}
									});

									if let Ok(scrolled_window) = child.downcast::<ScrolledWindow>() {
										scrolled_window.set_hexpand(false);
										scrolled_window.set_vexpand(false);
										scrolled_window.set_size_request(-1, 1);
										scrolled_window.set_max_content_height(1);
									}
								}
							}
						}
					}
				}
				
				if let Some((window_width, height_window)) = dock_window.adjust_window_height() {
					dock_window.set_pos_inscreen(&*c_display, window_width, height_window, *pos_inscreen.borrow());
				}
			}),
		);

		#[cfg(feature = "demo_mode")]
		#[cfg_attr(docsrs, doc(cfg(feature = "demo_mode")))]
		notebook.append_page(
			&{
				let vbox = GtkBox::new(gtk::Orientation::Vertical, 0);
				vbox.style_context().add_class("vinotebookpage");
				vbox.set_valign(gtk::Align::Fill);
				vbox.set_halign(gtk::Align::Baseline);

				{
					let vimetr = ViMeter::new_visender(
						app_config.clone(),
						"# Demo (time: 80, value: 0.7)",
						dock_window.allocation().width(),
						200,
						Some(vigraph_surface.clone()),
						c_transparent,
					);
					vbox.pack_start(&*vimetr, false, false, 0);
					glib::timeout_add_local(std::time::Duration::from_millis(80), move || {
						vimetr.push_next_and_queue_draw(0.7, 0.7, 1.0, 0.0, 0.0);

						ControlFlow::Continue
					});
				}
				{
					let vimetr = ViMeter::new_visender(
						app_config.clone(),
						"# Demo (time: 10ms, step: 0.1)",
						dock_window.allocation().width(),
						200,
						Some(vigraph_surface.clone()),
						c_transparent,
					);
					let data = RefCell::new(0.0);
					vbox.pack_start(&*vimetr, false, false, 0);
					glib::timeout_add_local(std::time::Duration::from_millis(10), move || {
						let mut w = RefCell::borrow_mut(&data);
						vimetr.push_next_and_queue_draw(*w, *w, 1.0, 0.0, 0.0);

						*w += 0.1;
						if *w >= 1.0 {
							*w = 0.0;
						}

						ControlFlow::Continue
					});
				}
				{
					let vimetr = ViMeter::new_visender(
						app_config.clone(),
						"# Demo (time: 1ms, step: 0.01)",
						dock_window.allocation().width(),
						200,
						Some(vigraph_surface.clone()),
						c_transparent,
					);
					let data = RefCell::new(0.0);
					vbox.pack_start(&*vimetr, false, false, 0);
					glib::timeout_add_local(std::time::Duration::from_millis(1), move || {
						let mut w = RefCell::borrow_mut(&data);
						vimetr.push_next_and_queue_draw(*w, *w, 1.0, 0.0, 0.0);

						*w += 0.01;
						if *w >= 1.0 {
							*w = 0.0;
						}

						ControlFlow::Continue
					});
				}
				{
					let vimetr = ViMeter::new_visender(
						app_config.clone(),
						"# Demo (time: 1ms, step: 0.001)",
						dock_window.allocation().width(),
						200,
						Some(vigraph_surface.clone()),
						c_transparent,
					);
					let data = RefCell::new(0.0);
					vbox.pack_start(&*vimetr, false, false, 0);
					glib::timeout_add_local(std::time::Duration::from_millis(1), move || {
						let mut w = RefCell::borrow_mut(&data);
						vimetr.push_next_and_queue_draw(*w, *w, 1.0, 0.0, 0.0);

						*w += 0.001;
						if *w >= 1.0 {
							*w = 0.0;
						}

						ControlFlow::Continue
					});
				}

				vbox.pack_end(&ViLabel::new((), &**app_config, "Notice: This page does not contain any useful information and is for debugging purposes only.", Weight::Bold)
					.set_margin_start(4)
					.set_margin_bottom(3)
					.set_wrap(true)
					.set_wrap_mode(WrapMode::Word)
					.set_max_width_chars(45)
					.set_align(Align::Center)
					.connect_nonblack_background(0.0, 0.0, 0.0, c_transparent), false, false, 0);
				vbox.set_visible(true);

				let scrolled_window = ScrolledWindow::builder().build();
				scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);  // Disable horizontal scrolling
				scrolled_window.set_hexpand(false);
				scrolled_window.set_vexpand(false);
				scrolled_window.set_size_request(-1, 1);
				scrolled_window.set_max_content_height(1);

				scrolled_window.set_child(Some(&vbox));
				scrolled_window.set_visible(true);
				scrolled_window
			},
			Some(&ViLabel::new(
				"head_vinotebook",
				&**app_config,
				"demo",
				Weight::Bold,
			)),
		);
		notebook.append_page(
			&{
				let vbox = GtkBox::new(gtk::Orientation::Vertical, 0);
				vbox.style_context().add_class("vinotebookpage");
				vbox.set_valign(gtk::Align::Fill);
				vbox.set_halign(gtk::Align::Baseline);

				let sensors: LMSensors = lm_sensors::Initializer::default()
					.initialize()
					.map_err(|e| anyhow!("{:?}", e))
					.unwrap(); // TODO REFACTORING ME?;

				for chip in sensors.chip_iter(None) {
					vbox.pack_start(
						&ViLabel::new(
							"info_vitextmeter",
							&**app_config,
							&format!("# {} ({})", chip, chip.bus()),
							(),
						)
						.set_margin_top(4)
						.set_margin_start(4)
						.set_margin_bottom(4)
						.set_align(Align::Start)
						.connect_nonblack_background(0.0, 0.0, 0.0, c_transparent),
						false,
						false,
						0,
					);

					println!("{} ({})", chip, chip.bus());

					// Print all features of the current chip.
					for feature in chip.feature_iter() {
						if let Some(name) = feature.name().transpose().unwrap() {
							println!("    {}: {}", name, feature);

							#[derive(Debug, Clone, Default)]
							struct Value<'a> {
								input: Option<(f64, Rc<SubFeatureRef<'a>>)>,
								max: Option<(f64, Rc<SubFeatureRef<'a>>)>,
								crit: Option<(f64, Rc<SubFeatureRef<'a>>)>,
								high: Option<(f64, Rc<SubFeatureRef<'a>>)>,
							}

							let mut c_value = Value::default();
							for sub_feature in feature.sub_feature_iter() {
								let sub_feature = Rc::new(sub_feature);
								if let Some(Ok(name)) = sub_feature.name() {
									if name.ends_with("input") {
										if let Ok(value) = sub_feature.value() {
											let v = value.raw_value();
											if v != 0.0 && v < 65261.0 && v > -273.0 {
												c_value.input = (v, sub_feature).into();
											}
										}
									} else if name.ends_with("max") {
										if let Ok(value) = sub_feature.value() {
											let v = value.raw_value();
											if v != 0.0 && v < 65261.0 && v > -273.0 {
												c_value.max = (v, sub_feature).into();
											}
										}
									} else if name.ends_with("high") {
										if let Ok(value) = sub_feature.value() {
											let v = value.raw_value();
											if v != 0.0 && v < 65261.0 && v > -273.0 {
												c_value.high = (v, sub_feature).into();
											}
										}
									} else if name.ends_with("crit") {
										if let Ok(value) = sub_feature.value() {
											let v = value.raw_value();
											if v != 0.0 && v < 65261.0 && v > -273.0 {
												c_value.crit = (v, sub_feature).into();
											}
										}
									}
								}
							}
							if c_value.input.is_some() {
								let vimetr = ViMeter::new_visender(
									app_config.clone(),
									name,
									dock_window.allocation().width(),
									200,
									Some(vigraph_surface.clone()),
									c_transparent,
								);

								vbox.pack_start(&*vimetr, false, false, 0);

								for _ in 0..400 {
									if let Some((input, sub_in)) = &c_value.input {
										if let Some((crit_or_max, sub_crit_or_max)) =
											c_value.crit.as_ref().or(c_value.max.as_ref())
										{
											#[inline]
											const fn map(
												x: f64,
												in_min: f64,
												in_max: f64,
												out_min: f64,
												out_max: f64,
											) -> f64 {
												(x - in_min) * (out_max - out_min)
													/ (in_max - in_min) + out_min
											}

											let v = sub_in.raw_value().unwrap();
											let graph = map(v, 0.0, *crit_or_max, 0.0, 1.0);
											vimetr.push_next_and_queue_draw(
												v,
												graph,
												*crit_or_max,
												*crit_or_max,
												0.0,
											);
										} else {
											vimetr.push_next_and_queue_draw(
												sub_in.raw_value().unwrap(),
												(),
												(),
												0.0,
												0.0,
											);
										}
									}
									//std::thread::sleep_ms(1);
								}
							}
						}
					}
				}

				if let Some(version) = sensors.version() {
					vbox.pack_end(
						&ViLabel::new(
							(),
							&**app_config,
							&format!("Version: {}", version),
							Weight::Bold,
						)
						.set_margin_start(4)
						.set_margin_bottom(4)
						.set_wrap(true)
						.set_wrap_mode(WrapMode::Word)
						.set_max_width_chars(45)
						.set_align(Align::Center)
						.connect_nonblack_background(0.0, 0.0, 0.0, c_transparent),
						false,
						false,
						0,
					);
				}
				vbox.set_visible(true);
				let scrolled_window = ScrolledWindow::builder().build();
				scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic); // Disable horizontal scrolling
				scrolled_window.set_hexpand(false);
				scrolled_window.set_vexpand(false);
				scrolled_window.set_size_request(-1, 1);
				scrolled_window.set_max_content_height(1);

				scrolled_window.set_child(Some(&vbox));
				scrolled_window.set_visible(true);
				scrolled_window
			},
			Some(&ViLabel::new(
				"head_vinotebook",
				&**app_config,
				"lm_sensors",
				Weight::Bold,
			)),
		);

		notebook.append_page(
			&{
				let vbox = GtkBox::new(gtk::Orientation::Vertical, 0);
				vbox.style_context().add_class("vinotebookpage");
				vbox.set_valign(gtk::Align::Fill);
				vbox.set_halign(gtk::Align::Baseline);

				let scrolled_window = ScrolledWindow::builder().build();
				scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
				scrolled_window.set_hexpand(false);
				scrolled_window.set_vexpand(false);
				scrolled_window.set_size_request(-1, 50);
				scrolled_window.set_max_content_height(50);

				scrolled_window.set_child(Some(&vbox));
				vbox.set_visible(true);
				scrolled_window.set_visible(true);
				scrolled_window
			},
			Some(&ViLabel::new(
				"head_vinotebook",
				&**app_config,
				"test",
				Weight::Bold,
			)),
		);

		for i in 1..notebook.n_pages() {
			if let Some(child) = notebook.nth_page(Some(i)) {
				if let Some(tab_label) = notebook.tab_label(&child) {
					if let Some(label) = tab_label.downcast_ref::<ViLabel>() {
						let style = label.style_context();
						if !style.has_class("notfirst_head_vinotebook") {
							style.add_class("notfirst_head_vinotebook");
						}
					}
				}
			}
		}

		if let Some(child) = notebook.nth_page(Some(0)) {
			if let Some(tab_label) = notebook.tab_label(&child) {
				if let Some(label) = tab_label.downcast_ref::<ViLabel>() {
					label.style_context().add_class("first_head_vinotebook");
				}
			}
		}

		vbox.pack_start(&notebook, true, true, 0);
		notebook.set_visible(true);

		notebook
	};

	vbox.pack_end(
		&ViDockHead::new(
			app_config.clone(),
			name_window,
			UPPERCASE_PKG_VERSION,
			c_transparent,
		),
		true,
		true,
		0,
	); // expand: true, fill: true

	dock_window.set_child(Some(&vbox));
	vbox.set_visible(true);

	spawn_keyboard_thread(esender);

	dock_window.connect_show(
		enc!((pos_inscreen, c_display, dock_window, notebook) move |_| {
			trace!("connect_show: ");
			glib::MainContext::default().spawn_local(enc!((notebook) async move {
				let nid = notebook.n_pages();
				notebook.insert_page(
					&{
						let scrolled_window = ScrolledWindow::builder().build();
						scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
						scrolled_window.set_hexpand(false);
						scrolled_window.set_vexpand(false);
						scrolled_window.set_size_request(-1, 1);
						scrolled_window.set_max_content_height(1);

						scrolled_window.set_visible(true);
						scrolled_window
					},
					None::<&ViLabel>,
					Some(nid),
				);

				notebook.set_page(nid as _);
				notebook.set_page(0);
				notebook.remove_page(Some(nid));
			}));

			dock_window.set_pos_inscreen(&*c_display, (), (), *pos_inscreen.borrow());
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
					AppEvents::Keyboard(AppKeyboardEvents::Num1) => notebook.set_page(0),
					AppEvents::Keyboard(AppKeyboardEvents::Num2) => notebook.set_page(1),
					AppEvents::Keyboard(AppKeyboardEvents::Num3) => notebook.set_page(2),
					AppEvents::Keyboard(AppKeyboardEvents::Num4) => notebook.set_page(3),
					AppEvents::Keyboard(AppKeyboardEvents::Num5) => notebook.set_page(4),
					AppEvents::Keyboard(AppKeyboardEvents::Num6) => notebook.set_page(5),
					AppEvents::Keyboard(AppKeyboardEvents::Num7) => notebook.set_page(6),
					AppEvents::Keyboard(AppKeyboardEvents::Num8) => notebook.set_page(7),
					AppEvents::Keyboard(AppKeyboardEvents::Num9) => notebook.set_page(8),
					AppEvents::Keyboard(AppKeyboardEvents::KeypadA) => {
						let mut a_page = notebook.current_page().unwrap_or(1);
						if a_page == 0 {
							a_page = notebook.n_pages() - 1;
						}else {
							a_page -= 1;
						}

						notebook.set_page(a_page as _);
					},
					AppEvents::Keyboard(AppKeyboardEvents::KeypadD) => {
						let mut a_page = notebook.current_page().unwrap_or(0) + 1;
						if a_page >= notebook.n_pages() {
							a_page = 0;
						}

						notebook.set_page(a_page as _);
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
					AppEvents::Keyboard(AppKeyboardEvents::KeypadPlus) => {},
					AppEvents::Keyboard(AppKeyboardEvents::KeypadMinus) => {},
					AppEvents::MoveDockWindowToNextPosition | AppEvents::Keyboard(AppKeyboardEvents::DoubleShift) => {
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
								("view-conceal-symbolic", "Hide | Show (Shift and F8)"),
								(
									"sidebar-show-right-symbolic-rtl",
									"Next position (Left Shift and Right Shift)",
								),
								("system-shutdown-symbolic", "Exit (Shift and Esc)")
							];
							let vihotkey = ViHotkeyItems::new(
								&*app_config,
								"# Hot keys",
								arr.into_iter(),
								c_transparent,
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
