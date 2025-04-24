use crate::core::display::ViGraphDisplayInfo;
use crate::core::dock_window::{PosINScreen, ViDockWindow};
use crate::widgets::indicator::ViIndicator;
use crate::widgets::primitives::label::ViLabel;
use anyhow::Result as anyhowResult;
use gtk::Box as GtkBox;
use gtk::glib::{ControlFlow, ExitCode};
use gtk::prelude::*;
use gtk::{Application, DrawingArea, cairo, glib};
use rand::random_range;
use std::sync::{Arc, Mutex};

mod widgets;
mod core {
	pub mod constuppercase;
	pub mod display;
	pub mod dock_window;
	pub mod gtk_codegen;
	pub mod maybe;
}

const WINDOW_WIDTH: i32 = 240;
const WINDOW_HEIGHT: i32 = 50;

fn draw_peak_graph(da: &DrawingArea, cr: &cairo::Context, data: &[f64]) {
	let allocation = da.allocation();
	let width = allocation.width() as f64;
	let height = allocation.height() as f64;

	cr.set_source_rgba(0.0, 0.0, 0.0, 0.5);
	cr.rectangle(0.0, 0.0, width, height);
	let _e = cr.fill();

	cr.set_source_rgba(0.0, 1.0, 0.0, 1.0);
	cr.set_line_width(1.5);

	let x_step = width / (data.len() - 1) as f64;

	for (enumerate, a) in data.iter().enumerate() {
		let x = enumerate as f64 * x_step;
		let y = height * (1.0 - a); // Инвертируем y, чтобы 0 был внизу

		if enumerate == 0 {
			cr.move_to(x, y);
		} else {
			cr.line_to(x, y);
		}
	}

	let _e = cr.stroke();
}

const APP_ID: &str = "com.ulinkot.ryzenpmmeter";
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const UPPERCASE_PKG_NAME: &str = const_ascii_uppercase!(PKG_NAME);

fn main() -> anyhowResult<ExitCode> {
	env_logger::try_init()?;

	let application = Application::new(Some(APP_ID), Default::default());
	application.connect_activate(move |app| {
		let c_display = ViGraphDisplayInfo::new(0).unwrap();
		let dock_window = ViDockWindow::new(app, UPPERCASE_PKG_NAME, WINDOW_WIDTH, WINDOW_HEIGHT);
		dock_window.connect_transparent_background(&c_display, 0.5);

		let vbox = GtkBox::new(gtk::Orientation::Vertical, 2);
		{
			let label = ViLabel::new(UPPERCASE_PKG_NAME)
				.set_margin(2)
				.connect_background(0.0, 1.0, 0.0, 0.5);

			vbox.pack_start(&label, true, true, 0); // expand: true, fill: true
		}
		{
			vbox.pack_start(
				&ViLabel::new("CPU Family: Raven").set_margin(2),
				true,
				true,
				0,
			); // expand: true, fill: true
		}
		{
			vbox.pack_start(
				&ViLabel::new("SMU BIOS Interface Version: 5").set_margin(2),
				true,
				true,
				0,
			); // expand: true, fill: true
		}
		{
			vbox.pack_start(
				&ViLabel::new("PM Table Version: 1e0004").set_margin(2),
				true,
				true,
				0,
			); // expand: true, fill: true
		}

		vbox.pack_start(
			&ViIndicator::new("TDP: 90", "MAX: 90", "AVG: 90"),
			false,
			false,
			2,
		);

		{
			let graph_area = DrawingArea::new();
			graph_area.set_size_request(WINDOW_WIDTH, 30);

			graph_area.connect_draw(move |da, cr| {
				draw_peak_graph(da, cr, &[1.0]);
				false.into()
			});

			vbox.pack_start(&graph_area, true, true, 0);
			dock_window.add(&vbox);
		}

		vbox.pack_start(
			&ViIndicator::new("TDP: 90", "MAX: 90", "AVG: 90"),
			false,
			false,
			2,
		);

		{
			let graph_area = DrawingArea::new();
			graph_area.set_size_request(200, 30);
			graph_area.connect_draw(move |da, cr| {
				draw_peak_graph(da, cr, &[1.0]);
				false.into()
			});

			vbox.pack_start(&graph_area, true, true, 0);
			dock_window.add(&vbox);
		}

		vbox.pack_start(
			&ViIndicator::new("TDP: 90", "MAX: 90", "AVG: 90"),
			false,
			false,
			2,
		);

		{
			let graph_area = DrawingArea::new();
			graph_area.set_size_request(200, 30);

			let arc = Arc::new(Mutex::new(vec![1.0]));
			let arc2 = arc.clone();
			graph_area.connect_draw(move |da, cr| {
				draw_peak_graph(da, cr, &arc2.lock().unwrap());

				false.into()
			});

			vbox.pack_start(&graph_area, true, true, 0);

			glib::timeout_add_local(std::time::Duration::from_millis(60 / 30), move || {
				{
					let mut lock = arc.lock().unwrap();

					while lock.len() > 20 {
						let _e = lock.remove(0);
					}

					lock.push(random_range(0.0..1.0));
				}

				graph_area.queue_draw();
				ControlFlow::Continue
			});

			dock_window.add(&vbox);
		}

		dock_window.add(&vbox);

		dock_window.show_all();
		dock_window.set_pos_inscreen(&c_display, PosINScreen::Center);

		dock_window.connect_show(|_| {});
	});

	Ok(application.run())
}
