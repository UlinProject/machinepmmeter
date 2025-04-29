use crate::config::Config;
use crate::core::display::ViGraphDisplayInfo;
use crate::core::dock_window::{PosINScreen, ViDockWindow};
use crate::core::keyboard::KeyboardListener;
use crate::core::keyboard::key::Key;
use crate::widgets::ViMeter;
use crate::widgets::dock_head::ViDockHead;
use crate::widgets::hotkeys::ViHotkeys;
use crate::widgets::primitives::icon_menuitem::ViIconMenuItem;
use crate::widgets::primitives::label::ViLabel;
use anyhow::anyhow;
use anyhow::{Context, Result as anyhowResult};
use appindicator3::traits::AppIndicatorExt;
use appindicator3::{Indicator, IndicatorCategory, IndicatorStatus};
use async_channel::{Receiver, Sender};
use clap::Parser;
use enclose::enc;
use gtk::gdk::{Monitor, Screen};
use gtk::gio::prelude::ApplicationExtManual;
use gtk::gio::traits::ApplicationExt;
use gtk::glib::ExitCode;
use gtk::prelude::WidgetExt;
use gtk::traits::{
	BoxExt, ContainerExt, CssProviderExt, GtkMenuItemExt, GtkWindowExt, MenuShellExt,
};
use gtk::{Align, Application, glib};
use gtk::{Box as GtkBox, CssProvider};
use lm_sensors::{LMSensors, SubFeatureRef};
use log::{error, info, trace, warn};
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

mod config;
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

const APP_NAME: &str = "machinepmmeter";
const UPPERCASE_APP_NAME: &str = const_ascii_uppercase!("machinepmmeter");
const APP_ID: &str = "com.ulinkot.machinepmmeter";
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const UPPERCASE_PKG_NAME: &str = const_ascii_uppercase!(PKG_NAME);

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const UPPERCASE_PKG_VERSION: &str = const_ascii_uppercase!(PKG_VERSION);

const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Parser, Debug)]
#[clap(
	name = "machinepmmeter",
	about = "A tool to monitor Machine power consumption"
)]
pub struct Cli {
	/// Path to the TOML configuration file
	#[clap(short, long, value_parser, default_value = None)]
	config: Option<PathBuf>,

	/// Allow saving default config if it doesn't exist
	#[clap(long, value_parser, default_value = "true")]
	allow_save_default_config: bool,
}

fn main() -> anyhowResult<ExitCode> {
	env_logger::try_init()?;
	let cli = Cli::parse();

	let config = Config::search_default_path(&cli, |config_path| {
		info!(
			"#[config file] open: {:?}, allow_save_default_config: {:?}",
			config_path, cli.allow_save_default_config
		);
		let config = {
			let context = || format!("Open config file {:?}.", &cli.config);
			let config = fs::read_to_string(config_path).map_or_else(
				|e| match cli.allow_save_default_config {
					false => Err(e).with_context(context),
					true => {
						let config = Config::default();

						Ok(config)
					}
				},
				|rdata| toml::from_str(&rdata).with_context(context),
			);

			Rc::new(config?)
		};

		Ok(config)
	})?;
	trace!("#[config file] current: {:?}", config);

	gtk::init()?;
	let c_display = Rc::new(ViGraphDisplayInfo::new(
		config.get_window_config().get_num_monitor(),
	)?);
	let defcss = {
		let a_css = CssProvider::new();
		a_css.load_from_data(include_bytes!("../style/def.css"))?;

		a_css
	};

	let application = Application::new(Some(APP_ID), Default::default());
	application.connect_activate(enc!((config) move |app| {
		let name_window = config.get_name_or_default();

		let (tx_keyboardevents, rx_keyboardevents) = async_channel::bounded(18);

		let mut is_keyboard_allowed = false;
		build_ui(app, name_window, &config, &c_display, &defcss, &mut is_keyboard_allowed, tx_keyboardevents.clone(), rx_keyboardevents);
		build_menu(is_keyboard_allowed, tx_keyboardevents);
	}));

	Ok(application.run())
}

enum KeyboardEvents {
	ShiftF8,
	KpPlus,
	KpMinus,
	DoubleShift,
	Escape,
}

enum Events {
	Keyboard(KeyboardEvents),
	HideOrShow,
	Close,
	NextPosition,
	AppendViHotkey(Vec<(&'static str, &'static str)>),
}

fn build_menu(is_keyboard_allowed: bool, sender: Sender<Events>) {
	let menu = gtk::Menu::new();
	{
		let menu_item = ViIconMenuItem::new(
			"view-conceal-symbolic",
			match is_keyboard_allowed {
				true => "Hide | Show (Shift and F8)",
				false => "Hide | Show ",
			},
		);

		menu_item.connect_activate(enc!((sender) move |_| {
			let _e = sender.send_blocking(Events::HideOrShow);
		}));
		menu.append(&*menu_item);
		menu_item.show_all();
	}
	{
		let menu_item = ViIconMenuItem::new(
			"sidebar-show-right-symbolic-rtl",
			match is_keyboard_allowed {
				true => "Next position (Left Shift and Right Shift)",
				false => "Next position ",
			},
		);

		menu_item.connect_activate(enc!((sender) move |_| {
			let _e = sender.send_blocking(Events::NextPosition);
		}));
		menu.append(&*menu_item);
		menu_item.show_all();
	}
	{
		let separator = gtk::SeparatorMenuItem::new();
		menu.append(&separator);
		separator.show_all();
	}
	{
		let menu_item = ViIconMenuItem::new(
			"system-shutdown-symbolic",
			match is_keyboard_allowed {
				true => "Exit (Shift and Esc)",
				false => "Exit",
			},
		);

		menu_item.connect_activate(enc!((sender) move |_| {
			let _e = sender.send_blocking(Events::Close);
		}));
		menu.append(&*menu_item);
		menu_item.show_all();
	}

	let icon = "help-about-symbolic";
	let indicator = Indicator::new(APP_ID, icon, IndicatorCategory::ApplicationStatus);
	indicator.set_status(IndicatorStatus::Active);
	indicator.set_menu(Some(&menu));
	indicator.set_attention_icon_full(icon, PKG_DESCRIPTION);

	gtk::main();
}

#[allow(clippy::too_many_arguments)]
fn build_ui(
	app: &gtk::Application,
	name_window: &str,
	config: &Rc<Config>,
	c_display: &Rc<ViGraphDisplayInfo>,
	defcss: &CssProvider,
	is_keyboard_allowed: &mut bool,

	sender: Sender<Events>,
	receiver: Receiver<Events>,
) {
	trace!("#[gui] Start initialization, name: {:?}", name_window);
	gtk::StyleContext::add_provider_for_screen(
		AsRef::<Screen>::as_ref(c_display as &ViGraphDisplayInfo),
		defcss,
		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);

	let dock_window = Rc::new(ViDockWindow::new(app, name_window, &**config));
	let (c_transparent, _is_transparent_mode) =
		config.get_window_config().get_transparent().map_or_else(
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
	vbox.pack_start(
		&ViDockHead::new(&**config, name_window, UPPERCASE_PKG_VERSION, c_transparent),
		true,
		true,
		0,
	); // expand: true, fill: true

	/*{
		vbox.pack_start(
			&ViLabel::new("head_info", &**config, "CPU Family: Raven", ())
				.set_margin_top(8)
				.set_margin_start(4)
				.set_margin_bottom(3)
				.set_align(Align::Start)
				.connect_nonblack_background(0.0, 0.0, 0.0, c_transparent),
			true,
			true,
			0,
		); // expand: true, fill: true
	}*/
	/*{
		vbox.pack_start(
			&ViLabel::new("head_info", &**config, "SMU BIOS Interface Version: 5", ())
				.set_margin_start(4)
				.set_margin_bottom(3)
				.set_align(Align::Start)
				.connect_nonblack_background(0.0, 0.0, 0.0, c_transparent),
			true,
			true,
			0,
		); // expand: true, fill: true
	}*/
	/*{
		vbox.pack_start(
			&ViLabel::new("head_info", &**config, "PM Table Version: 1e0004", ())
				.set_margin_start(4)
				.set_margin_bottom(3)
				.set_align(Align::Start)
				.connect_nonblack_background(0.0, 0.0, 0.0, c_transparent),
			true,
			true,
			0,
		); // expand: true, fill: true
	}*/

	let sensors: Arc<LMSensors> = Arc::new(
		lm_sensors::Initializer::default()
			.initialize()
			.map_err(|e| anyhow!("{:?}", e))
			.unwrap(),
	); // TODO REFACTORING ME?;

	{
		vbox.pack_start(
			&ViLabel::new("head_info", &**config, "View: lm_sensors", ())
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
				&**config,
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
						config.clone(),
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

	/*{
		let vimetr = ViMeter::new_visender(
			config.clone(),
			"# TDP",
			dock_window.allocation().width(),
			200,
			c_transparent,
		);
		vbox.pack_start(&*vimetr, false, false, 0);
		glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
			vimetr.push_next_and_queue_draw(random_range(1..9) as f64 * 0.10);

			ControlFlow::Continue
		});
	}

	{
		let vimetr = ViMeter::new_visender(
			config.clone(),
			"# VRM",
			dock_window.allocation().width(),
			200,
			c_transparent,
		);
		vbox.pack_start(&*vimetr, false, false, 0);
		glib::timeout_add_local(std::time::Duration::from_millis(600), move || {
			vimetr.push_next_and_queue_draw(random_range(0.8..0.9));

			ControlFlow::Continue
		});
	}

	{
		let vimetr = ViMeter::new_visender(
			config.clone(),
			"# VOLTAGE",
			dock_window.allocation().width(),
			200,
			c_transparent,
		);
		vbox.pack_start(&*vimetr, false, false, 0);
		glib::timeout_add_local(std::time::Duration::from_millis(600), move || {
			vimetr.push_next_and_queue_draw(random_range(0.8..0.9));

			ControlFlow::Continue
		});
	}*/
	dock_window.add(&*vbox);

	std::thread::spawn(enc!((sender)move || {
		let keyboard_listener = KeyboardListener::listen::<6>(
			|key_table| {
				key_table[0].set_key(Key::ShiftRight);
				key_table[1].set_key(Key::ShiftLeft);
				key_table[2].set_key(Key::F8);
				key_table[3].set_key(Key::KpPlus);
				key_table[4].set_key(Key::KpMinus);
				key_table[5].set_key(Key::Escape);
			},
			enc!((sender) move |state_array, _key, _state| {
				match (
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
							.send_blocking(Events::Keyboard(KeyboardEvents::ShiftF8));
					}
					((true, false) | (false, true), false, true, false, false) => {
						// L/R SHIFT + KpPlus
						let _e = sender
							.send_blocking(Events::Keyboard(KeyboardEvents::KpPlus));
					}
					((true, false) | (false, true), false, false, true, false) => {
						// L/R SHIFT + KpMinus
						let _e = sender
							.send_blocking(Events::Keyboard(KeyboardEvents::KpMinus));
					}
					((true, false) | (false, true), false, false, false, true) => {
						// L/R SHIFT + Escape
						let _e = sender
							.send_blocking(Events::Keyboard(KeyboardEvents::Escape));
					}
					((true, true), ..) => {
						// L+R SHIFT
						let _e = sender
							.send_blocking(Events::Keyboard(KeyboardEvents::DoubleShift));
					}
					_ => {}
				}
			}),
			|| {
				let _e = sender.send_blocking(Events::AppendViHotkey(vec![
					("view-conceal-symbolic", "Hide | Show (Shift and F8)"),
					(
						"sidebar-show-right-symbolic-rtl",
						"Next position (Left Shift and Right Shift)",
					),
					("system-shutdown-symbolic", "Exit (Shift and Esc)"),
				]));
			},
		);

		match keyboard_listener {
			Ok(()) => {}
			Err(e) => {
				error!(
					"#[global keyboard] Error initializing global keyboard listener, keyboard shortcuts not available. {}",
					e
				);
			}
		};
	}));

	let pos_inscreen = Rc::new(RefCell::new(config.get_window_config().get_pos()));
	dock_window.connect_show(enc!((pos_inscreen, c_display, dock_window) move |_| {
		let pos_inscreen: PosINScreen = *pos_inscreen.borrow();

		dock_window.set_pos_inscreen(&*c_display, pos_inscreen);
	}));
	dock_window.connect_resize_mode_notify(enc!((pos_inscreen, c_display, dock_window) move |_| {
		let pos_inscreen: PosINScreen = *pos_inscreen.borrow();

		dock_window.set_pos_inscreen(&*c_display, pos_inscreen);
	}));
	dock_window.connect_screen_changed(
		enc!((pos_inscreen, config, c_display) move |dock_window, screen| {
			let pos_inscreen: PosINScreen = *pos_inscreen.borrow();
			let mut owned_motitor = None;
			let c_monitor: &Monitor = ViGraphDisplayInfo::as_ref(&*c_display);
			let monitor: &Monitor = screen
				.map(|a| a.display())
				.and_then(|a| {
					owned_motitor = a.monitor(config.get_window_config().get_num_monitor());
					owned_motitor.as_ref()
			}).unwrap_or(c_monitor);

			dock_window.set_pos_inscreen(monitor, pos_inscreen);
		}),
	);

	glib::MainContext::default().spawn_local(
		enc!((c_display, dock_window, pos_inscreen, vbox, config) async move {
			while let Ok(event) = receiver.recv().await {
				match event {
					Events::HideOrShow | Events::Keyboard(KeyboardEvents::ShiftF8) if dock_window.is_visible() => {
						dock_window.hide();
					},
					Events::HideOrShow | Events::Keyboard(KeyboardEvents::ShiftF8) => {
						dock_window.show();
					},
					Events::Close | Events::Keyboard(KeyboardEvents::Escape) => {
						dock_window.close();
						gtk::main_quit();
					},
					Events::Keyboard(KeyboardEvents::KpPlus) => {},
					Events::Keyboard(KeyboardEvents::KpMinus) => {},
					Events::NextPosition | Events::Keyboard(KeyboardEvents::DoubleShift) => {
						let new_pos = { // NEXT POS IN SCREEN
							let mut write = pos_inscreen.borrow_mut();
							let new_pos = write.next();
							*write = new_pos;

							new_pos
						};
						dock_window.set_pos_inscreen(&*c_display, new_pos);
					},
					Events::AppendViHotkey(arr) => {
						let vihotkey = ViHotkeys::new(
							&*config,
							"# Hot keys",
							arr.into_iter(),
							c_transparent,
						);
						vbox.add(&vihotkey);
						vbox.show_all();
					}
				}
			}
		}),
	);

	glib::MainContext::default().spawn_local(enc!((dock_window) async move {
		dock_window.show_all();
	}));
}
