use crate::config::Config;
use crate::core::display::ViGraphDisplayInfo;
use crate::core::dock_window::ViDockWindow;
use crate::core::keyboard_listener::KeyboardListener;
use crate::widgets::ViMeter;
use crate::widgets::dock_head::ViDockHead;
use crate::widgets::primitives::label::ViLabel;
use anyhow::{Context, Result as anyhowResult};
use clap::Parser;
use enclose::enc;
use gtk::gdk::{Monitor, Screen};
use gtk::gio::prelude::ApplicationExtManual;
use gtk::gio::traits::ApplicationExt;
use gtk::glib::{ControlFlow, ExitCode};
use gtk::prelude::WidgetExt;
use gtk::traits::{BoxExt, ContainerExt, CssProviderExt};
use gtk::{Align, Application, glib};
use gtk::{Box as GtkBox, CssProvider};
use log::{info, trace, warn};
use rand::random_range;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use core::keyboard_listener::Key;

mod config;
mod widgets;
mod core {
	pub mod constuppercase;
	pub mod display;
	pub mod dock_window;
	pub mod gtk_codegen;
	pub mod maybe;
	pub mod keyboard_listener;
}

const APP_ID: &str = "com.ulinkot.machinepmmeter";
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const UPPERCASE_PKG_NAME: &str = const_ascii_uppercase!(PKG_NAME);

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const UPPERCASE_PKG_VERSION: &str = const_ascii_uppercase!(PKG_VERSION);

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
	application.connect_activate(enc!((config, c_display) move |app| {
		let name_window = config.get_name_or_default();
		trace!("#[gui] Start initialization, name: {:?}", name_window);
		gtk::StyleContext::add_provider_for_screen(AsRef::<Screen>::as_ref(&c_display as &ViGraphDisplayInfo), &defcss, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

		let dock_window = ViDockWindow::new(app, name_window, &*config);
		let transparent = config.get_window_config().get_transparent().map_or_else(
			|| 1.0,
			|level| match !dock_window.connect_transparent_background(&*c_display, level).is_true() {
				false => {
					warn!("#[gui] Transparency was expected, but the system does not support it");

					1.0
				},
				true => level
			}
		);
		
		let vbox = GtkBox::new(gtk::Orientation::Vertical, 0);
		vbox.pack_start(&ViDockHead::new(&*config, name_window, UPPERCASE_PKG_VERSION, transparent), true, true, 0); // expand: true, fill: true
		{
			vbox.pack_start(
				&ViLabel::new("head_info", &*config, "CPU Family: Raven", ())
				.set_margin_top(8)
					.set_margin_start(4)
					.set_margin_bottom(3)
					.set_align(Align::Start)
					.connect_nonblack_background(0.0, 0.0, 0.0, transparent),
				true,
				true,
				0,
			); // expand: true, fill: true
		}
		{
			vbox.pack_start(
				&ViLabel::new("head_info", &*config, "SMU BIOS Interface Version: 5", ())
				.set_margin_start(4)
				.set_margin_bottom(3)
				.set_align(Align::Start)
					.connect_nonblack_background(0.0, 0.0, 0.0, transparent),
				true,
				true,
				0,
			); // expand: true, fill: true
		}
		{
			vbox.pack_start(
				&ViLabel::new("head_info", &*config, "PM Table Version: 1e0004", ())
				.set_margin_start(4)
				.set_margin_bottom(3)
				.set_align(Align::Start)
					.connect_nonblack_background(0.0, 0.0, 0.0, transparent),
				true,
				true,
				0,
			); // expand: true, fill: true
		}

		{
			let vimetr = ViMeter::new_visender(config.clone(), "# TDP", dock_window.allocation().width(), 200, transparent);
			vbox.pack_start(
				&*vimetr,
				false,
				false,
				0,
			);
			glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
				vimetr.push_next_and_queue_draw(random_range(1..9) as f64 * 0.10);

				ControlFlow::Continue
			});
		}

		{
			let vimetr = ViMeter::new_visender(config.clone(), "# VRM", dock_window.allocation().width(), 200, transparent);
			vbox.pack_start(
				&*vimetr,
				false,
				false,
				0,
			);
			glib::timeout_add_local(std::time::Duration::from_millis(600), move || {
				vimetr.push_next_and_queue_draw(random_range(0.8..0.9));

				ControlFlow::Continue
			});
		}

		{
			let vimetr = ViMeter::new_visender(config.clone(), "# VOLTAGE", dock_window.allocation().width(), 200, transparent);
			vbox.pack_start(
				&*vimetr,
				false,
				false,
				0,
			);
			glib::timeout_add_local(std::time::Duration::from_millis(600), move || {
				vimetr.push_next_and_queue_draw(random_range(0.8..0.9));

				ControlFlow::Continue
			});
		}
		dock_window.add(&vbox);
		dock_window.show_all();

		dock_window.set_pos_inscreen(&*c_display, config.get_window_config().get_pos());
		dock_window.connect_screen_changed(enc!((config, c_display) move |dock_window, screen| {
			let mut owned_motitor = None;
			let c_monitor: &Monitor = ViGraphDisplayInfo::as_ref(&*c_display);
			let monitor: &Monitor = screen
				.map(|a| a.display())
				.and_then(|a| {
					owned_motitor = a.monitor(config.get_window_config().get_num_monitor());
					owned_motitor.as_ref()
			}).unwrap_or(c_monitor);

			dock_window.set_pos_inscreen(monitor, config.get_window_config().get_pos());
		}));
		
		let (tx, rx) = async_channel::bounded::<()>(10);
		std::thread::spawn(move || {
		let keyboard_listener = KeyboardListener::listen::<3>(
			|key_table| {
				key_table[0].set_key(Key::ShiftRight);
				key_table[1].set_key(Key::ShiftLeft);
				key_table[2].set_key(Key::F8);
			},
			move |state_array, _key, _state| {
				if (state_array[0].is_pressed() || state_array[1].is_pressed())
					&& state_array[2].is_pressed()
				{
					println!("SHIFT + F8");
					tx.send_blocking(());
				}
			},
		);});
		let pos = config.get_window_config().get_pos();
		let c_display = c_display.clone();
		glib::MainContext::default().spawn_local(async move {
			while let Ok(text) = rx.recv().await {
				if dock_window.is_visible() {
					dock_window.hide();
				}else {
					dock_window.show_all();
				}
				dock_window.set_pos_inscreen(&*c_display, pos);
			}
		});
	}));

	Ok(application.run())
}
