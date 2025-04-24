use crate::{
	__gen_transparent_gtk_type,
	core::constopt::ConstOption,
	fn_const_option,
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
	pub fn new<'a, 'b, MAX: ConstOption<&'a str>, AVG: ConstOption<&'b str>>(
		value: &str,
		max: MAX,
		avg: AVG,
	) -> Self {
		let hbox = Box::new(Orientation::Horizontal, 0);

		hbox.pack_start(&ViLabel::new(value, 10), true, true, 0);

		fn_const_option!(max, |max| hbox.pack_start(
			&ViLabel::new(max, 10),
			true,
			true,
			0
		));
		fn_const_option!(avg, |avg| hbox.pack_start(
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
