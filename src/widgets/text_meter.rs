use crate::{
	__gen_transparent_gtk_type,
	config::{ColorConfig, FontConfig},
	core::maybe::Maybe,
	maybe,
	widgets::primitives::{color_block::ViColorBlock, label::ViLabel},
};
use gtk::{
	Align, Box, Orientation,
	ffi::GtkBox,
	traits::{BoxExt, WidgetExt},
};

#[repr(transparent)]
#[derive(Debug)]
pub struct ViTextMeter(Box);

__gen_transparent_gtk_type! {
	#[sys(GtkBox)]
	ViTextMeter(
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

impl ViTextMeter {
	pub fn new<'a, 'b>(
		config: impl AsRef<FontConfig> + AsRef<ColorConfig> + Copy,

		value: &'_ str,
		max: impl Maybe<&'a str>,
		avg: impl Maybe<&'b str>,

		transparent: f64,
	) -> Self {
		let hbox = Box::new(Orientation::Horizontal, 0);
		hbox.connect_draw(move |window, cr| {
			let allocation = window.allocation();
			cr.set_source_rgba(0.220, 0.220, 0.220, transparent);

			cr.rectangle(
				0.0,
				0.0,
				allocation.width().into(),
				allocation.height().into(),
			);
			let _e = cr.fill();

			false.into()
		});
		let margin_top = 3 + 3;
		let margin_bottom = 3;

		hbox.pack_start(
			&ViColorBlock::new(2, 0).connect_color::<true>(config, |c| c.green(), transparent),
			false,
			true,
			2,
		);

		hbox.pack_start(
			&ViLabel::new("arg_ViTextMeter", config, value)
				.set_align(Align::Center)
				.set_margin_top(margin_top)
				.set_margin_bottom(margin_bottom),
			true,
			true,
			0,
		);

		maybe!(max, |max| hbox.pack_start(
			&ViLabel::new("arg_ViTextMeter", config, max)
				.set_align(Align::Center)
				.set_margin_top(margin_top)
				.set_margin_bottom(margin_bottom),
			true,
			true,
			0
		));
		maybe!(avg, |avg| hbox.pack_start(
			&ViLabel::new("arg_ViTextMeter", config, avg)
				.set_align(Align::Center)
				.set_margin_top(margin_top)
				.set_margin_bottom(margin_bottom),
			true,
			true,
			0
		));

		Self(hbox)
	}

	#[inline]
	pub fn set_margin(self, margin: i32) -> Self {
		self.0.set_margin(margin);

		self
	}

	#[inline]
	pub fn set_margin_top(self, margin: i32) -> Self {
		self.0.set_margin_top(margin);

		self
	}

	#[inline]
	pub fn set_margin_start(self, margin: i32) -> Self {
		self.0.set_margin_start(margin);

		self
	}

	#[inline]
	pub fn set_margin_end(self, margin: i32) -> Self {
		self.0.set_margin_end(margin);

		self
	}

	#[inline]
	pub fn set_margin_bottom(self, margin: i32) -> Self {
		self.0.set_margin_bottom(margin);

		self
	}
}
