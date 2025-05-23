use crate::__gen_transparent_gtk_type;
use crate::app::config::FontAppConfig;
use crate::core::maybe::Maybe;
use crate::maybe;
use crate::widgets::primitives::hotkeyitem::ViHotkeyItem;
use crate::widgets::primitives::label::ViLabel;
use gtk::Align;
use gtk::Box;
use gtk::Orientation;
use gtk::ffi::GtkBox;
use gtk::pango::Weight;
use gtk::traits::BoxExt;
use gtk::traits::WidgetExt;
use std::ops::Deref;

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
	pub fn new<'item, 'n>(
		f_app_config: impl AsRef<FontAppConfig> + Copy,
		head: impl Maybe<&'n str>,
		items: impl Iterator<Item = &'item (&'item str, &'item str, &'item str)>,
	) -> Self {
		let all = Box::new(Orientation::Vertical, 0);
		all.set_valign(gtk::Align::Fill);
		all.set_halign(gtk::Align::Baseline);

		maybe!((head) {
			all.pack_start(
				&ViLabel::new((), f_app_config, head, Weight::Bold)
					.set_margin_top(4)
					.set_margin_start(4)
					.set_margin_bottom(3)
					.set_align(Align::Start),
				false,
				false,
				0,
			);
		});
		{
			let shortcasthbox = Box::new(Orientation::Vertical, 2);
			for (icon, text, key) in items {
				shortcasthbox.pack_start(
					&ViHotkeyItem::new(f_app_config, icon, text, key),
					false,
					false,
					0,
				);
			}
			all.pack_start(&shortcasthbox, false, false, 0);

			shortcasthbox.set_visible(true);
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
