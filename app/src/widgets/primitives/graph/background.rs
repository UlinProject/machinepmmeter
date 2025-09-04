use anyhow::Result as anyhowResult;
use gtk::cairo;
use gtk::cairo::Context;
use gtk::cairo::ImageSurface;
use std::cell::RefCell;
use std::rc::Rc;

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
