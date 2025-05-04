use crate::{
	__gen_transparent_gtk_type, app::config::AppConfig, core::maybe::Maybe, maybe,
	widgets::primitives::label::ViLabel,
};
use enclose::enc;
use gtk::{
	Align, Box,
	ffi::GtkBox,
	traits::{BoxExt, StyleContextExt, WidgetExt},
};
use std::rc::Rc;

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

		head.connect_draw(enc!((app_config)move |window, cr| {
			let head_color = app_config.get_window_app_config().get_head_color();

			if head_color.is_notblack(transparent) {
					let (r, g, b, a) = head_color.into_rgba(transparent);
					let (width, height) = {
						let allocation = window.allocation();

						(allocation.width().into(), allocation.height().into())
					};

					cr.set_source_rgba(r, g, b, a);
					cr.rectangle(
						0.0,
						0.0,
						width,
						height
					);
					let _e = cr.fill();
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
