use crate::config::{ColorConfig, Config};
use crate::core::display::ViGraphDisplayInfo;
use crate::core::dock_window::ViDockWindow;
use crate::widgets::indicator::ViIndicator;
use crate::widgets::primitives::dock_head::ViDockHead;
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
use gtk::{Align, Application, DrawingArea, cairo, glib};
use gtk::{Box as GtkBox, CssProvider};
use log::{info, trace, warn};
use rand::random_range;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

mod config;
mod widgets;
mod core {
	pub mod constuppercase;
	pub mod display;
	pub mod dock_window;
	pub mod gtk_codegen;
	pub mod maybe;
}

fn draw_peak_graph(
	color: impl AsRef<ColorConfig>,
	da: &DrawingArea,
	cr: &cairo::Context,
	data: &[f64],
	transparent: f64,
) {
	let color = color.as_ref();
	let allocation = da.allocation();
	let width = allocation.width().into();
	let height = allocation.height().into();

	{
		// background
		cr.move_to(0.0, 0.0);
		cr.set_source_rgba(0.255, 0.255, 0.255, transparent);

		cr.rectangle(0.0, 0.0, width, height);
		let _e = cr.fill();
	}

	let num_horizontal_lines = 10;
	let num_vertical_lines = 10;

	cr.set_source_rgba(0.8, 0.8, 0.8, transparent * 0.5);
	cr.set_line_width(0.1);

	for i in 1..num_horizontal_lines {
		let y = height / num_horizontal_lines as f64 * i as f64;
		cr.move_to(0.0, y);
		cr.line_to(width, y);
		let _e = cr.stroke();
	}
	for i in 1..num_vertical_lines {
		let x = width / num_vertical_lines as f64 * i as f64;
		cr.move_to(x, 0.0);
		cr.line_to(x, height);
		let _e = cr.stroke();
	}

	cr.set_line_width(2.0);

	let x_step = width / (data.len() - 1) as f64;

	for (enumerate, a) in data.iter().enumerate() {
		let x = enumerate as f64 * x_step;
		let y = height * (1.0 - a);

		if enumerate == 0 {
			cr.move_to(0.0, height * (1.0 - data[0])); // Start at the first data point
		} else {
			cr.line_to(x, y);
		}

		let (r, g, b) = if a > &0.9 {
			color.orange()
		} else if a > &0.7 {
			color.orange()
		} else {
			color.green()
		};

		cr.set_source_rgba(
			r as f64 / 255.0,
			g as f64 / 255.0,
			b as f64 / 255.0,
			transparent,
		);
	}

	let _e = cr.stroke();
}

const APP_ID: &str = "com.ulinkot.ryzenpmmeter";
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const UPPERCASE_PKG_NAME: &str = const_ascii_uppercase!(PKG_NAME);

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const UPPERCASE_PKG_VERSION: &str = const_ascii_uppercase!(PKG_VERSION);

#[derive(Parser, Debug)]
#[clap(
	name = "ryzenpmmeter",
	about = "A tool to monitor Ryzen power consumption"
)]
struct Cli {
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
				&ViLabel::new("head_info", &*config, "CPU Family: Raven")
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
				&ViLabel::new("head_info", &*config, "SMU BIOS Interface Version: 5")
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
				&ViLabel::new("head_info", &*config, "PM Table Version: 1e0004")
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
				&ViLabel::new("info_viindicator", &*config, "# TDP")
				.set_margin_top(4)
				.set_margin_start(4)
				.set_margin_bottom(3)
				.set_align(Align::Start)
					.connect_nonblack_background(0.0, 0.0, 0.0, transparent),
				true,
				true,
				0,
			); // expand: true, fill: true
		}
		vbox.pack_start(
			&ViIndicator::new(&*config, "90", "MAX: 90", "AVG: 90", transparent),
			false,
			false,
			0,
		);

		{
			let allocation = dock_window.allocation();
			let graph_area = DrawingArea::new();
			graph_area.set_margin_bottom(6);
			graph_area.set_size_request(allocation.width(), 42);

			graph_area.connect_draw(enc!((config) move |da, cr| {
				draw_peak_graph(&*config, da, cr, &[1.0], transparent);
				false.into()
			}));

			vbox.pack_start(&graph_area, true, true, 0);
			dock_window.add(&vbox);
		}

		{
			vbox.pack_start(
				&ViLabel::new("info_viindicator", &*config, "# VRM")
				.set_margin_top(4)
				.set_margin_start(4)
				.set_margin_bottom(3)
				.set_align(Align::Start)
					.connect_nonblack_background(0.0, 0.0, 0.0, transparent),
				true,
				true,
				0,
			); // expand: true, fill: true
		}
		vbox.pack_start(
			&ViIndicator::new(&*config, "90", "MAX: 90", "AVG: 90", transparent),
			false,
			false,
			0,
		);

		{
			let allocation = dock_window.allocation();
			let graph_area = DrawingArea::new();
			graph_area.set_size_request(allocation.width(), 42);
			graph_area.set_margin_bottom(6);
			graph_area.connect_draw(enc!((config) move |da, cr| {
				draw_peak_graph(&*config, da, cr, &[1.0], transparent);
				false.into()
			}));

			vbox.pack_start(&graph_area, true, true, 0);
			dock_window.add(&vbox);
		}

		{
			vbox.pack_start(
				&ViLabel::new("info_viindicator", &*config, "# VOLTAGE")
				.set_margin_top(4)
				.set_margin_start(4)
				.set_margin_bottom(3)
				.set_align(Align::Start)
					.connect_nonblack_background(0.0, 0.0, 0.0, transparent),
				true,
				true,
				0,
			); // expand: true, fill: true
		}
		vbox.pack_start(
			&ViIndicator::new(&*config, "90", "MAX: 90", "AVG: 90", transparent),
			false,
			false,
			0,
		);

		{
			let allocation = dock_window.allocation();
			let graph_area = DrawingArea::new();
			graph_area.set_margin_bottom(6);
			graph_area.set_size_request(allocation.width(), 42);

			let arc = Arc::new(Mutex::new(vec![1.0]));
			let arc2 = arc.clone();
			graph_area.connect_draw(enc!((config) move |da, cr| {
				draw_peak_graph(&*config, da, cr, &arc2.lock().unwrap(), transparent);

				false.into()
			}));

			vbox.pack_start(&graph_area, true, true, 0);

			glib::timeout_add_local(std::time::Duration::from_millis(60), move || {
				{
					let mut lock = arc.lock().unwrap();

					while lock.len() > 120 {
						let _e = lock.remove(0);
					}

					lock.push(random_range(0.0..1.0));
				}

				graph_area.queue_draw();
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
	}));

	Ok(application.run())
}
