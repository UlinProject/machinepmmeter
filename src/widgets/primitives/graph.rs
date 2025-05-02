use crate::__gen_transparent_gtk_type;
use crate::app::config::AppConfig;
use anyhow::Result as anyhowResult;
use enclose::enc;
use gtk::DrawingArea;
use gtk::cairo;
use gtk::cairo::Context;
use gtk::cairo::ImageSurface;
use gtk::ffi::GtkDrawingArea;
use gtk::traits::WidgetExt;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;

#[repr(transparent)]
#[derive(Debug)]
pub struct ViGraph(DrawingArea);

__gen_transparent_gtk_type! {
	#[sys(GtkDrawingArea)]
	ViGraph(
		new |a: DrawingArea| {
			Self(a)
		},
		ref |sself| {
			&sself.0
		},
		into |sself| {
			sself.0
		},
	)
}

impl ViGraph {
	pub fn new_graphsender(
		app_config: Rc<AppConfig>,

		general_background_surface: Option<ViGraphSurface>,
		width: i32,
		height: i32,
		len: usize,
		transparent: f64,
	) -> ViGraphSender {
		let rc_data = Rc::new(RefCell::new(VecDeque::from(vec![0.0; len])));

		let graph_area = DrawingArea::new();
		graph_area.set_size_request(width, height);

		let background_surface = general_background_surface.map_or_else(Default::default, |a| a);
		graph_area.connect_realize(enc!((background_surface) move |da| {
			let (width, height) = {
				let allocation = da.allocation();

				(allocation.width(), allocation.height())
			};
			let _e = background_surface.draw_or_get(width, height, transparent, |_|{});
		}));

		graph_area.connect_draw(
			enc!((rc_data, background_surface, app_config) move |da, cr| {
				let data = RefCell::borrow(&rc_data);

				let (width, height) = {
					let allocation = da.allocation();

					(allocation.width(), allocation.height())
				};
				if background_surface.draw_or_get(width, height, transparent, |surface| {
					let _e = cr.set_source_surface(surface, 0.0, 0.0);
					let _e = cr.paint();
				}).is_err() {
					cr.set_source_rgba(0.255, 0.255, 0.255, transparent);
					cr.rectangle(0.0, 0.0, width as _, height as _);
					let _e = cr.fill();
				}

				let (width, height): (f64, f64) = (width.into(), height.into());
				let (r, g, b, transparent) = {
					let color_config = app_config.get_color_app_config();
					let a_forcolor = data.back().copied().unwrap_or_default();

					if a_forcolor >= 0.85 {
						color_config.red().into_rgba(transparent)
					} else if a_forcolor >= 0.75 {
						color_config.orange().into_rgba(transparent)
					} else {
						color_config.green().into_rgba(transparent)
					}
				};

				let x_step = width / (len - 1) as f64;
				{// shadow
					let (sr, sg, sb, st): (f64, f64, f64, f64) = (0.8, 0.8, 0.8, 0.2);
					let yoffset = 1.0;
					let width = 3.8;
					if let Some(first_a) = data.front() {
						cr.move_to(0.0, height * (1.0 - first_a) + yoffset);
						cr.set_source_rgba(sr, sg, sb, st);
						cr.set_line_width(width);

						for (i, a) in data.iter().enumerate() {
							let x = (i + 1) as f64 * x_step;
							let y = height * (1.0 - a) + yoffset;

							cr.line_to(x, y);
						}
					}
					let _e = cr.stroke();
				}

				if let Some(first_a) = data.front() {
					cr.move_to(0.0, height * (1.0 - first_a));
					cr.set_source_rgba(r, g, b, transparent);
					cr.set_line_width(1.5);

					for (i, a) in data.iter().enumerate() {
						let x = (i + 1) as f64 * x_step;
						let y = height * (1.0 - a);

						cr.line_to(x, y);
					}
				}
				let _e = cr.stroke();

				false.into()
			}),
		);

		ViGraphSender(rc_data, Self(graph_area))
	}
}

pub struct ViGraphSender(Rc<RefCell<VecDeque<f64>>>, ViGraph);

impl Deref for ViGraphSender {
	type Target = ViGraph;

	fn deref(&self) -> &Self::Target {
		&self.1
	}
}

impl ViGraphSender {
	pub fn push_next_and_queue_draw(&self, v: f64) {
		self.push_next(v);
		self.queue_draw();
	}

	pub fn push_next(&self, v: f64) {
		let mut lock = RefCell::borrow_mut(&self.0);

		lock.pop_front();
		lock.push_back(v);
	}

	#[inline]
	pub fn queue_draw(&self) {
		self.1.queue_draw();
	}
}

#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct ViGraphSurface(Rc<RefCell<Option<ImageSurface>>>);

impl ViGraphSurface {
	pub fn draw_or_get<R>(
		&self,
		width: i32,
		height: i32,
		transparent: f64,
		next: impl FnOnce(&ImageSurface) -> R,
	) -> anyhowResult<R> {
		let mut w = self.0.borrow_mut();
		{
			if let Some(ref surface) = *w {
				if surface.width() == width && surface.height() == height {
					return Ok(next(surface));
				}
			}

			let surface = ImageSurface::create(cairo::Format::ARgb32, width, height)?;
			let cr = Context::new(&surface)?;
			let (width, height) = (width.into(), height.into());

			let _e = cr.save();
			cr.move_to(0.0, 0.0);
			cr.set_source_rgba(0.255, 0.255, 0.255, transparent);
			cr.rectangle(0.0, 0.0, width, height);
			let _e = cr.fill();

			let c_horizontal_lines = 10 / 2;
			let c_vertical_lines = 10;

			cr.set_source_rgba(0.8, 0.8, 0.8, transparent);
			cr.set_line_width(0.1);

			for i in 1..c_horizontal_lines {
				let y = height / c_horizontal_lines as f64 * i as f64;

				cr.move_to(0.0, y);
				cr.line_to(width, y);
				let _e = cr.stroke();
			}
			for i in 1..c_vertical_lines {
				let x = width / c_vertical_lines as f64 * i as f64;

				cr.move_to(x, 0.0);
				cr.line_to(x, height);
				let _e = cr.stroke();
			}
			let _e = cr.restore();

			let result = Ok(next(&surface));
			*w = Some(surface);

			result
		}
	}
}
