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

		general_background_surface: Option<ViGraphBackgroundSurface>,
		width: i32,
		height: i32,
		len: usize,
		transparent: f64,
	) -> ViGraphSender {
		let rc_data = Rc::new(RefCell::new(VecDeque::from(vec![0.0; len])));
		let cache_surface = Rc::new(RefCell::new(ViGraphCachedSurface::empty()));

		let graph_area = DrawingArea::new();
		graph_area.set_size_request(width, height);

		let background_surface = general_background_surface.map_or_else(Default::default, |a| a);
		
		#[cfg(feature = "graph-background-cache")]
		#[cfg_attr(docsrs, doc(cfg(feature = "graph-background-cache")))] {
			graph_area.connect_realize(enc!((background_surface) move |da| {
				let (width, height) = {
					let allocation = da.allocation();

					(allocation.width(), allocation.height())
				};
				let _e = background_surface.draw_or_get(width, height, transparent, |_|{ Ok(()) });
			}));
		}

		graph_area.connect_draw(
			enc!((rc_data, background_surface, app_config, cache_surface) move |da, in_cr| {
				let (width, height) = {
					let allocation = da.allocation();

					(allocation.width(), allocation.height())
				};

				if width <= 1 || height <= 1 {
					return true.into();
				}

				let mut w_cache_surface = RefCell::borrow_mut(&cache_surface);
				match w_cache_surface.get_or_recrate_surface(
					width, height,
					|cache_surface, is_always_redraw| {
						if *is_always_redraw {
							*is_always_redraw = false;
						}else {
							let _e = in_cr.set_source_surface(cache_surface, 0.0, 0.0);
							let _e = in_cr.paint();

							return Ok(true.into());
						}

						if let Ok(cr) = Context::new(cache_surface) {
							if background_surface.draw_or_get(width, height, transparent, |surface| {
								let _e = cr.set_source_surface(surface, 0.0, 0.0);
								let _e = cr.paint();

								Ok(())
							}).is_err() {
								cr.set_source_rgba(0.255, 0.255, 0.255, transparent);
								cr.rectangle(0.0, 0.0, width as _, height as _);

								let _e = cr.fill();
							}

							let data = RefCell::borrow(&rc_data);
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

							let _e = in_cr.set_source_surface(cache_surface, 0.0, 0.0);
							let _e = in_cr.paint();
						}

						Ok(false.into())
					}
				) {
					Ok(a) => a,
					Err(_e) => true.into(),
				}
			}),
		);

		ViGraphSender {
			data: rc_data,
			cache_surface,
			vi: Self(graph_area),
		}
	}
}

pub struct ViGraphSender {
	data: Rc<RefCell<VecDeque<f64>>>,
	cache_surface: Rc<RefCell<ViGraphCachedSurface>>,
	vi: ViGraph,
}

impl Deref for ViGraphSender {
	type Target = ViGraph;

	fn deref(&self) -> &Self::Target {
		&self.vi
	}
}

impl ViGraphSender {
	pub fn push_next_and_queue_draw(&self, v: f64) {
		self.push_next(v);

		ViGraphSender::queue_draw(self);
	}

	pub fn push_next(&self, v: f64) {
		let mut lock = RefCell::borrow_mut(&self.data);

		lock.pop_front();
		lock.push_back(v);
	}

	#[inline]
	pub fn queue_draw(&self) {
		{
			let mut w = RefCell::borrow_mut(&self.cache_surface);
			w.add_redraw();
		}
		self.vi.queue_draw();
	}
}

pub struct ViGraphCachedSurface {
	surface: Option<ImageSurface>,
	is_always_redraw: bool,
}

impl ViGraphCachedSurface {
	#[inline]
	pub const fn empty() -> Self {
		Self {
			surface: None,
			is_always_redraw: true,
		}
	}

	#[inline]
	pub fn add_redraw(&mut self) {
		self.is_always_redraw = true;
	}

	pub fn get_or_recrate_surface<R>(
		&mut self,
		width: i32,
		height: i32,
		next: impl FnOnce(&ImageSurface, &mut bool) -> anyhowResult<R>,
	) -> anyhowResult<R> {
		match self.surface {
			Some(ref a) if a.width() == width && a.height() == height => {
				next(a, &mut self.is_always_redraw)
			}
			_ => match ImageSurface::create(cairo::Format::ARgb32, width, height) {
				Ok(a) => {
					self.surface = Some(a);
					self.is_always_redraw = true;

					match self.surface {
						Some(ref a) => next(a, &mut self.is_always_redraw),
						_ => unimplemented!(),
					}
				}
				Err(e) => Err(e.into()),
			},
		}
	}
}

#[repr(transparent)]
#[derive(Debug, Clone, Default)]
#[cfg(feature = "graph-background-cache")]
#[cfg_attr(docsrs, doc(cfg(feature = "graph-background-cache")))]
pub struct ViGraphBackgroundSurface(Rc<RefCell<Option<ImageSurface>>>);

#[repr(transparent)]
#[derive(Debug, Clone, Default)]
#[cfg(not(feature = "graph-background-cache"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "graph-background-cache"))))]
pub struct ViGraphBackgroundSurface();

impl ViGraphBackgroundSurface {
	pub fn draw_or_get<R>(
		&self,
		width: i32,
		height: i32,
		transparent: f64,
		next: impl FnOnce(&ImageSurface) -> anyhowResult<R>,
	) -> anyhowResult<R> {
		#[cfg(feature = "graph-background-cache")]
		#[cfg_attr(docsrs, doc(cfg(feature = "graph-background-cache")))]
		let mut w = self.0.borrow_mut();
		{
			#[cfg(feature = "graph-background-cache")]
			#[cfg_attr(docsrs, doc(cfg(feature = "graph-background-cache")))]
			{
				if let Some(ref surface) = *w {
					if surface.width() == width && surface.height() == height {
						return next(surface);
					}
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

			let result = next(&surface);

			#[cfg(feature = "graph-background-cache")]
			#[cfg_attr(docsrs, doc(cfg(feature = "graph-background-cache")))]
			{
				*w = Some(surface);
			}

			result
		}
	}
}
