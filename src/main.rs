// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Denis Kotlyarov (Денис Котляров) <denis2005991@gmail.com>

use crate::app::about_dialog::AppAboutDialog;
use crate::app::cli::AppCli;
use crate::app::config::AppConfig;
use crate::app::tray_menu::{AppTrayMenu, AppTrayMenuItem};
use crate::core::display::ViGraphDisplayInfo;
use crate::core::dock_window::{PosINScreen, ViDockWindow};
use crate::core::keyboard::KeyboardListenerBuilder;
use crate::core::keyboard::key::Key;
use crate::widgets::ViMeter;
use crate::widgets::dock_head::ViDockHead;
use crate::widgets::hotkeys::ViHotkeyItems;
use crate::widgets::primitives::icon_menuitem::ViIconMenuItem;
use crate::widgets::primitives::label::ViLabel;
use anyhow::anyhow;
use anyhow::{Context, Result as anyhowResult};
use async_channel::{Receiver, Sender};
use clap::Parser;
use enclose::enc;
use glib::{ControlFlow, ExitCode};
use gtk::gdk::{Monitor, Screen};
use gtk::gio::prelude::ApplicationExtManual;
use gtk::gio::traits::ApplicationExt;
use gtk::prelude::WidgetExt;
use gtk::traits::{BoxExt, ContainerExt, CssProviderExt, GtkMenuItemExt, GtkWindowExt};
use gtk::{Align, Application};
use gtk::{Box as GtkBox, CssProvider};
use lm_sensors::{LMSensors, SubFeatureRef};
use log::{error, info, trace, warn};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::sync::Arc;

mod widgets;
mod core {
	pub mod constuppercase;
	pub mod display;
	pub mod dock_window;
	pub mod eight_bitcolor;
	pub mod gtk_codegen;
	pub mod keyboard;
	pub mod maybe;
}
pub mod app {
	pub mod about_dialog;
	pub mod cli;
	pub mod config;
	pub mod tray_menu;
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

	let (tx_appevents, rx_appevents) = async_channel::bounded(18);
	let rx_appevents = Rc::new(rx_appevents);
	let tray_menu = {
		// Tray menu
		let hide_or_show = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
			vi.connect_activate(enc!((tx_appevents) move |_| {
				let _e = tx_appevents.send_blocking(AppEvents::HideOrShow);
			}));
		}) as &'_ mut dyn FnMut(&'_ mut ViIconMenuItem);

		let next_position = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
			vi.connect_activate(enc!((tx_appevents) move |_| {
				let _e = tx_appevents.send_blocking(AppEvents::NextPosition);
			}));
		});

		let abouttheprogram = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
			vi.connect_activate(enc!((tx_appevents) move |_| {
				let _e = tx_appevents.send_blocking(AppEvents::AboutTheProgram);
			}));
		});

		let exit = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
			vi.connect_activate(enc!((tx_appevents) move |_| {
				let _e = tx_appevents.send_blocking(AppEvents::Exit);
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

		Rc::new(tray_menu)
	};

	let application = Application::new(Some(APP_ID), Default::default());
	application.connect_activate(enc!((app_config, rx_appevents, tray_menu) move |app| {
		if !tray_menu.is_connected() {
			error!(
				"#[global traymenu] Error initializing tray menu, tray menu will be unavailable.",
			);
		}

		let name_window = app_config.get_name_or_default();

		build_ui(app, name_window, &app_config, &c_display, &defcss, tx_appevents.clone(), rx_appevents.clone());
	}));
	
	application.run();
	Ok(())
}

enum KeyboardEvents {
	ShiftF8,
	KpPlus,
	KpMinus,
	DoubleShift,
	Escape,
}

enum AppEvents {
	Keyboard(KeyboardEvents),
	HideOrShow,
	AboutTheProgram,
	Exit,
	NextPosition,
	KeyboardListenerState(bool),
}

#[allow(clippy::too_many_arguments)]
fn build_ui(
	app: &gtk::Application,
	name_window: &str,
	app_config: &Rc<AppConfig>,
	c_display: &Rc<ViGraphDisplayInfo>,
	defcss: &CssProvider,

	sender: Sender<AppEvents>,
	receiver: Rc<Receiver<AppEvents>>,
) {
	trace!("#[gui] Start initialization, name: {:?}", name_window);
	gtk::StyleContext::add_provider_for_screen(
		AsRef::<Screen>::as_ref(c_display as &ViGraphDisplayInfo),
		defcss,
		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);

	let dock_window = Rc::new(ViDockWindow::new(app, name_window, &**app_config));
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

	let vbox = Rc::new(GtkBox::new(gtk::Orientation::Vertical, 0));
	vbox.set_valign(gtk::Align::Start);
	vbox.set_halign(gtk::Align::Baseline);

	vbox.pack_start(
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

	let sensors: LMSensors = lm_sensors::Initializer::default()
		.initialize()
		.map_err(|e| anyhow!("{:?}", e))
		.unwrap(); // TODO REFACTORING ME?;

	{
		vbox.pack_start(
			&ViLabel::new("head_info", &**app_config, "View: lm_sensors", ())
				.set_margin_top(8)
				.set_margin_start(4)
				.set_margin_bottom(3)
				.set_align(Align::Start)
				.connect_nonblack_background(0.0, 0.0, 0.0, c_transparent),
			true,
			true,
			0,
		); // expand: true, fill: true
	}

	// Print all chips.
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
			.set_margin_bottom(3)
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
									(x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
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

	{
		let vimetr = ViMeter::new_visender(
			app_config.clone(),
			"# Demo",
			dock_window.allocation().width(),
			200,
			c_transparent,
		);
		vbox.pack_start(&*vimetr, false, false, 0);
		glib::timeout_add_local(std::time::Duration::from_millis(80), move || {
			vimetr.push_next_and_queue_draw(0.7, 0.7, 1.0, 0.0, 0.0);

			ControlFlow::Continue
		});
	}
	dock_window.set_child(Some(&*vbox));
	vbox.set_visible(true);

	std::thread::spawn(enc!((sender)move || {
		let keyboard_listener = KeyboardListenerBuilder::with_len::<6>()
			.key_mapping(|key_mapping| {
				key_mapping[0].set_key(Key::ShiftRight);
				key_mapping[1].set_key(Key::ShiftLeft);
				key_mapping[2].set_key(Key::F8);
				key_mapping[3].set_key(Key::KpPlus);
				key_mapping[4].set_key(Key::KpMinus);
				key_mapping[5].set_key(Key::Escape);
			})
			.handler(enc!((sender) move |state_array, _key, _state| match (
					(
						state_array[0].is_pressed(), // ShiftRight
						state_array[1].is_pressed(), // ShiftLeft
					),
					state_array[2].is_pressed(), // F8
					state_array[3].is_pressed(), // KpPlus
					state_array[4].is_pressed(), // KpMinus
					state_array[5].is_pressed(), // Escape
				) {
					((true, false) | (false, true), true, false, false, false) => {
						// L/R SHIFT + F8
						let _e = sender
							.send_blocking(AppEvents::Keyboard(KeyboardEvents::ShiftF8));
					}
					((true, false) | (false, true), false, true, false, false) => {
						// L/R SHIFT + KpPlus
						let _e = sender
							.send_blocking(AppEvents::Keyboard(KeyboardEvents::KpPlus));
					}
					((true, false) | (false, true), false, false, true, false) => {
						// L/R SHIFT + KpMinus
						let _e = sender
							.send_blocking(AppEvents::Keyboard(KeyboardEvents::KpMinus));
					}
					((true, false) | (false, true), false, false, false, true) => {
						// L/R SHIFT + Escape
						let _e = sender
							.send_blocking(AppEvents::Keyboard(KeyboardEvents::Escape));
					}
					((true, true), ..) => {
						// L+R SHIFT
						let _e = sender
							.send_blocking(AppEvents::Keyboard(KeyboardEvents::DoubleShift));
					}
					_ => {}
				}
			)).on_startup(|| {
				let _e = sender.send_blocking(AppEvents::KeyboardListenerState(true));
			}).listen();

		if let Err(e) = keyboard_listener {
			error!(
				"#[global keyboard] Error initializing global keyboard listener, keyboard shortcuts not available. {}",
				e
			);
			let _e = sender.send_blocking(AppEvents::KeyboardListenerState(false));
		}
	}));

	let pos_inscreen = Rc::new(RefCell::new(app_config.get_window_app_config().get_pos()));
	dock_window.connect_show(enc!((pos_inscreen, c_display, dock_window) move |_| {
		let pos_inscreen: PosINScreen = *pos_inscreen.borrow();

		dock_window.set_pos_inscreen(&*c_display, pos_inscreen);
	}));

	dock_window.connect_resize_mode_notify(enc!((pos_inscreen, c_display, dock_window) move |_| {
		let pos_inscreen: PosINScreen = *pos_inscreen.borrow();

		dock_window.set_pos_inscreen(&*c_display, pos_inscreen);
	}));
	dock_window.connect_screen_changed(
		enc!((pos_inscreen, app_config, c_display) move |dock_window, screen| {
			let pos_inscreen: PosINScreen = *pos_inscreen.borrow();
			let mut owned_motitor = None;
			let c_monitor: &Monitor = ViGraphDisplayInfo::as_ref(&*c_display);
			let monitor: &Monitor = screen
				.map(|a| a.display())
				.and_then(|a| {
					owned_motitor = a.monitor(app_config.get_window_app_config().get_num_monitor());
					owned_motitor.as_ref()
			}).unwrap_or(c_monitor);

			dock_window.set_pos_inscreen(monitor, pos_inscreen);
		}),
	);

	glib::timeout_add_local(
		std::time::Duration::from_millis(2000),
		enc!((sender)move || {
			let _e = sender.send_blocking(AppEvents::KeyboardListenerState(true));

			ControlFlow::Continue
		}),
	);
	glib::timeout_add_local(
		std::time::Duration::from_millis(1500),
		enc!((sender)move || {
			let _e = sender.send_blocking(AppEvents::KeyboardListenerState(false));

			ControlFlow::Continue
		}),
	);

	glib::MainContext::default().spawn_local(
		enc!((c_display, dock_window, pos_inscreen, vbox, app_config) async move {
			let app_about_dialog = Rc::new(RefCell::new(None));
			let mut wdock_vihotkey = None;
			while let Ok(event) = receiver.recv().await {
				match event {
					AppEvents::HideOrShow | AppEvents::Keyboard(KeyboardEvents::ShiftF8) if dock_window.is_visible() => {
						dock_window.hide();
					},
					AppEvents::HideOrShow | AppEvents::Keyboard(KeyboardEvents::ShiftF8) => {
						dock_window.show();
					},
					AppEvents::Exit | AppEvents::Keyboard(KeyboardEvents::Escape) => {
						dock_window.close();
						gtk::main_quit();
					},
					AppEvents::AboutTheProgram => {
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
					AppEvents::Keyboard(KeyboardEvents::KpPlus) => {},
					AppEvents::Keyboard(KeyboardEvents::KpMinus) => {},
					AppEvents::NextPosition | AppEvents::Keyboard(KeyboardEvents::DoubleShift) => {
						let new_pos = { // NEXT POS IN SCREEN
							let mut write = pos_inscreen.borrow_mut();
							let new_pos = write.next();
							*write = new_pos;

							new_pos
						};
						dock_window.set_pos_inscreen(&*c_display, new_pos);
					},
					AppEvents::KeyboardListenerState(true) => {
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
							vihotkey.show_all();
							
							wdock_vihotkey = Some(vihotkey);
						}
					},
					AppEvents::KeyboardListenerState(false) => {
						if let Some(vihotkey) = wdock_vihotkey {
							vihotkey.set_visible(false);
							vbox.remove(&vihotkey);

							wdock_vihotkey = None;
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
