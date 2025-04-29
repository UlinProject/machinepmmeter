use crate::maybe;
use crate::{__gen_transparent_gtk_type, core::maybe::Maybe};
use gtk::MenuItem;
use gtk::ffi::GtkMenuItem;
use gtk::gio::Icon;
use gtk::traits::ContainerExt;
use std::ops::Deref;

#[repr(transparent)]
#[derive(Debug)]
pub struct ViIconMenuItem(MenuItem);

__gen_transparent_gtk_type! {
	#[sys(GtkMenuItem)]
	ViIconMenuItem(
		new |a: MenuItem| {
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

impl Deref for ViIconMenuItem {
	type Target = MenuItem;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ViIconMenuItem {
	pub fn new<'c>(icon: impl Maybe<&'c str>, label: &'_ str) -> Self {
		let menu_item = gtk::MenuItem::new();
		menu_item.set_child(Some(&{
			let g_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);

			let label = gtk::Label::new(Some(label));

			maybe!((icon) {
				if let Ok(icon) = Icon::for_string(icon) {
					g_box.add(&gtk::Image::from_gicon(&icon, gtk::IconSize::Menu));
				}
			});
			g_box.add(&label);

			g_box
		}));

		Self(menu_item)
	}
}
