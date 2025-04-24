use crate::{
	__gen_transparent_gtk_type, core::maybe::Maybe, maybe, widgets::primitives::{color_block::ViColorBlock, label::ViLabel}
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
	pub fn new<'a, 'b, MAX: Maybe<&'a str>, AVG: Maybe<&'b str>>(
		value: &str,
		max: MAX,
		avg: AVG,
	) -> Self {
		let hbox = Box::new(Orientation::Horizontal, 0);

		hbox.pack_start(&ViLabel::new(value, 10), true, true, 0);

		maybe!(max, |max| hbox.pack_start(
			&ViLabel::new(max, 10),
			true,
			true,
			0
		));
		maybe!(avg, |avg| hbox.pack_start(
			&ViLabel::new(avg, 10),
			true,
			true,
			0
		));

		hbox.pack_end(
			&ViColorBlock::new(20, 20).connect_background::<true>(0.0, 1.0, 0.0, 0.5),
			false,
			false,
			0,
		);

		Self(hbox)
	}
}
