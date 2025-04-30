use crate::{
	__gen_transparent_gtk_type,
	app::config::{FontAppConfig, WindowAppConfig},
	core::maybe::Maybe,
	maybe,
	widgets::primitives::label::ViLabel,
};
use gtk::{
	Align, Box,
	ffi::GtkBox,
	traits::{BoxExt, StyleContextExt, WidgetExt},
};

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
		app_config: impl AsRef<WindowAppConfig> + AsRef<FontAppConfig> + Copy,

		value: &'a str,
		version: impl Maybe<&'b str>,
		transparent: f64,
	) -> Self {
		let head = Box::new(gtk::Orientation::Horizontal, 0);
		head.style_context().add_class("namehead");
		let head_color = (app_config.as_ref() as &WindowAppConfig).get_head_color();

		if head_color.is_notblack(transparent) {
			let (r, g, b, a) = head_color.into_rgba(transparent);

			head.connect_draw(move |window, cr| {
				let allocation = window.allocation();
				cr.set_source_rgba(r, g, b, a);

				cr.rectangle(
					0.0,
					0.0,
					allocation.width().into(),
					allocation.height().into(),
				);
				let _e = cr.fill();

				false.into()
			});
		}

		let name_label = ViLabel::new("namehead_vilabel", app_config, value, ())
			.set_margin_start(4)
			.set_margin_top(2);
		head.pack_start(&name_label, false, true, 0); // expand: true, fill: true

		maybe!((version) {
			let version_label = ViLabel::new("versionhead_vilabel", app_config, version, ())
				.set_align(Align::End)
				.set_margin_top(2);

			head.pack_start(&version_label, true, true, 0); // expand: true, fill: true
		});

		Self(head)
	}
}
