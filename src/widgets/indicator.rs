use crate::{
	__gen_transparent_gtk_type,
	config::{ColorConfig, FontConfig},
	core::maybe::Maybe,
	maybe,
	widgets::primitives::{color_block::ViColorBlock, label::ViLabel},
};
use gtk::{Box, Orientation, ffi::GtkBox, traits::BoxExt};

#[repr(transparent)]
#[derive(Debug)]
pub struct ViIndicator(Box);

__gen_transparent_gtk_type! {
	#[sys(GtkBox)]
	ViIndicator(
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

impl ViIndicator {
	pub fn new<'a, 'b>(
		config: impl AsRef<FontConfig> + AsRef<ColorConfig> + Copy,

		value: &'_ str,
		max: impl Maybe<&'a str>,
		avg: impl Maybe<&'b str>,

		transparent: f64,
	) -> Self {
		let hbox = Box::new(Orientation::Horizontal, 0);
		hbox.pack_start(
			&ViLabel::new(config, value)
				.set_margin(10)
				.connect_nonblack_background(0.0, 0.0, 0.0, transparent),
			true,
			true,
			0,
		);

		maybe!(max, |max| hbox.pack_start(
			&ViLabel::new(config, max)
				.set_margin(10)
				.connect_nonblack_background(0.0, 0.0, 0.0, transparent),
			true,
			true,
			0
		));
		maybe!(avg, |avg| hbox.pack_start(
			&ViLabel::new(config, avg)
				.set_margin(10)
				.connect_nonblack_background(0.0, 0.0, 0.0, transparent),
			true,
			true,
			0
		));

		hbox.pack_end(
			&ViColorBlock::new(20, 20).connect_color::<true>(config, |c| c.green(), transparent),
			false,
			false,
			0,
		);

		Self(hbox)
	}
}
