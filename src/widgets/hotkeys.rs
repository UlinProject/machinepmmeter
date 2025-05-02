use std::ops::Deref;

use crate::__gen_transparent_gtk_type;
use crate::app::config::FontAppConfig;
use crate::core::maybe::Maybe;
use crate::maybe;
use crate::widgets::primitives::hotkeyitem::ViHotkeyItem;
use crate::widgets::primitives::label::ViLabel;
use gtk::traits::WidgetExt;
use gtk::Align;
use gtk::Box;
use gtk::Orientation;
use gtk::ffi::GtkBox;
use gtk::pango::Weight;
use gtk::traits::BoxExt;

#[repr(transparent)]
#[derive(Debug)]
pub struct ViHotkeyItems(Box);

__gen_transparent_gtk_type! {
	#[sys(GtkBox)]
	ViHotkeyItems(
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

impl ViHotkeyItems {
	pub fn new<'a, 'n>(
		f_app_config: impl AsRef<FontAppConfig> + Copy,
		head: impl Maybe<&'n str>,
		array: impl Iterator<Item = (&'a str, &'a str)>,
		c_transparent: f64,
	) -> Self {
		let all = Box::new(Orientation::Vertical, 0);

		maybe!((head) {
			all.pack_start(
				&ViLabel::new((), f_app_config, head, Weight::Bold)
					.set_margin_top(4)
					.set_margin_start(4)
					.set_margin_bottom(3)
					.set_align(Align::Start)
					.connect_nonblack_background(0.0, 0.0, 0.0, c_transparent),
				false,
				false,
				0,
			);
		});
		{
			let shortcasthbox = Box::new(Orientation::Vertical, 1);
			for (icon, text) in array {
				shortcasthbox.pack_start(
					&ViHotkeyItem::new(f_app_config, icon, text),
					false,
					false,
					0,
				);
			}
			all.pack_start(&shortcasthbox, false, false, 0);
		}
		all.set_visible(true);

		Self(all)
	}
}

impl Deref for ViHotkeyItems {
	type Target = Box;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
