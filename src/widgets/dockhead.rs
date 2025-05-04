use crate::{
	__gen_transparent_gtk_type, app::config::AppConfig, core::maybe::Maybe, maybe,
	widgets::primitives::label::ViLabel,
};
use enclose::enc;
use gtk::{
	cairo::{self, Context, ImageSurface}, ffi::GtkBox, traits::{BoxExt, StyleContextExt, WidgetExt}, Align, Box
};
use std::{cell::RefCell, rc::Rc};

#[repr(transparent)]
#[derive(Debug)]
pub struct ViDockHead(Box);

__gen_transparent_gtk_type! {
	#[sys(GtkBox)]
	ViDockHead(
		new |a: Box| {
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

impl ViDockHead {
	pub fn new<'a, 'b>(
		app_config: Rc<AppConfig>,

		value: &'a str,
		version: impl Maybe<&'b str>,
		transparent: f64,
	) -> Self {
		let head = Box::new(gtk::Orientation::Horizontal, 0);
		head.style_context().add_class("namehead");
		head.set_valign(gtk::Align::Center);
		head.set_halign(gtk::Align::Fill);
		
		let background: Rc<RefCell<Option<ImageSurface>>> = Default::default();
		head.connect_draw(enc!((app_config, background) move |window, in_cr| {
			let mut w = background.borrow_mut();
			let (width, height) = {
				let allocation = window.allocation();

				(allocation.width(), allocation.height())
			};
			if width <= 1 || height <= 1 {
				return false.into();
			}
			
			if let Some(ref surface) = *w {
				if surface.width() == width && surface.height() == height {
					let _e = in_cr.set_source_surface(surface, 0.0, 0.0);
					let _e = in_cr.paint();
					
					return false.into();
				}
			}
			
			let head_color = app_config.get_window_app_config().get_head_color();
			if let Ok(surface) = ImageSurface::create(cairo::Format::ARgb32, width, height) {
				if let Ok(cr) = Context::new(&surface) {
					let (width, height) = (width.into(), height.into());
					{ // background
						let (r, g, b, a) = head_color.into_rgba(transparent);

						cr.set_source_rgba(r, g, b, a);
						cr.rectangle(
							0.0,
							0.0,
							width,
							height
						);
						let _e = cr.fill();
					}

					{ // grid
						let (r, g, b, a) = (0.0, 0.0, 0.0, 0.1);
						let line_width = 0.7;
						let step = 8.0;

						cr.set_source_rgba(r, g, b, a);
						cr.set_line_width(line_width);

						let x_offset = (width % (step+line_width)) / 2.0;
						let y_offset = (height % (step+line_width)) / 2.0;

						let mut x = x_offset;
						while x <= width {
							cr.move_to(x, 0.0);
							cr.line_to(x, height);

							x += step + line_width;
						}

						let mut y = y_offset;
						while y <= height {
							cr.move_to(0.0, y);
							cr.line_to(width, y);

							y += step +line_width;
						}

						let _e = cr.stroke();
					}
					
					let _e = in_cr.set_source_surface(&surface, 0.0, 0.0);
					let _e = in_cr.paint();
					
					*w = Some(surface);
				}
			}

			false.into()
		}));

		head.pack_start(
			&ViLabel::new("namehead_vilabel", &*app_config, value, ())
				.set_align(Align::Start)
				.set_margin_start(4)
				.set_margin_top(2),
			false,
			true,
			0,
		);

		maybe!((version) {
			let version_label = ViLabel::new("versionhead_vilabel", &*app_config, version, ())
				.set_align(Align::End)
				.set_margin_end(3)
				.set_margin_top(2);

			head.pack_end(&version_label, true, true, 0);
		});

		head.set_visible(true);

		Self(head)
	}
}
