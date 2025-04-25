use crate::{
	__gen_transparent_gtk_type,
	config::{FontConfig, WindowConfig},
	core::maybe::Maybe,
	maybe,
	widgets::primitives::label::ViLabel,
};
use gtk::{
	ffi::GtkBox, traits::{BoxExt, WidgetExt}, Align, Box
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
		config: impl AsRef<WindowConfig> + AsRef<FontConfig> + Copy,

		value: &'a str,
		version: impl Maybe<&'b str>,
		transparent: f64,
	) -> Self {
		let head = Box::new(gtk::Orientation::Horizontal, 0);
		let (red, green, blue) = (config.as_ref() as &WindowConfig).get_head_color();

		if red != 0 || green != 0 || blue != 0 || transparent != 1.0 {
			let red = (red as f64) / 255.0;
			let green = (green as f64) / 255.0;
			let blue = (blue as f64) / 255.0;

			head.connect_draw(move |window, cr| {
				let allocation = window.allocation();
				cr.set_source_rgba(red, green, blue, transparent);

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

		let name_label = ViLabel::new("namehead_vilabel", config, value, ())
			.set_margin_start(4)
			.set_margin_top(2);
		head.pack_start(&name_label, false, true, 0); // expand: true, fill: true

		maybe!((version) {
			let version_label = ViLabel::new("versionhead_vilabel", config, version, ())
				.set_align(Align::End)
				.set_margin_top(2);

			head.pack_start(&version_label, true, true, 0); // expand: true, fill: true
		});

		Self(head)
	}
}
