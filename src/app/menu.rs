use std::ops::Deref;

use crate::__gen_transparent_gtk_type;
use crate::core::maybe::Maybe;
use crate::maybe;
use crate::widgets::primitives::icon_menuitem::ViIconMenuItem;
use appindicator3::Indicator;
pub use appindicator3::IndicatorCategory;
use appindicator3::IndicatorStatus;
use appindicator3::traits::AppIndicatorExt;
use gtk::traits::WidgetExt;
use gtk::Menu;
use gtk::ffi::GtkMenu;
use gtk::traits::MenuShellExt;

#[repr(transparent)]
#[derive(Debug)]
pub struct AppTrayMenu(Menu);

__gen_transparent_gtk_type! {
	#[sys(GtkMenu)]
	AppTrayMenu(
		new |a: Menu| {
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

impl Deref for AppTrayMenu {
	type Target = Menu;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub enum AppTrayMenuItem<'i, 'v, F>
where
	F: FnMut(&'_ mut ViIconMenuItem),
{
	IconItem {
		icon: &'i str,
		value: &'v str,
		init: F,
	},
	Item {
		value: &'v str,
		init: F,
	},
	Separator,
}

impl<'i, 'v, F> AppTrayMenuItem<'i, 'v, F>
where
	F: FnMut(&'_ mut ViIconMenuItem),
{
	#[inline]
	pub const fn separator() -> Self {
		Self::Separator
	}

	#[inline]
	pub const fn item(value: &'v str, init: F) -> Self {
		Self::Item { value, init }
	}

	#[inline]
	pub const fn icon_item(icon: &'i str, value: &'v str, init: F) -> Self {
		Self::IconItem { icon, value, init }
	}
}

impl AppTrayMenu {
	pub fn new<'i, 'v, 'desc, 'title, F: FnMut(&'_ mut ViIconMenuItem)>(
		id: &'_ str,
		icon: &'_ str,
		title: impl Maybe<&'title str>,
		desc: impl Maybe<&'desc str>,
		items: impl Iterator<Item = AppTrayMenuItem<'i, 'v, F>>,
	) -> Self {
		let menu = gtk::Menu::new();
		for item in items {
			match item {
				AppTrayMenuItem::IconItem {
					icon,
					value,
					mut init,
				} => {
					let mut menu_item = ViIconMenuItem::new(icon, value);

					init(&mut menu_item);
					menu.append(&*menu_item);
					menu_item.show();
				}
				AppTrayMenuItem::Item { value, mut init } => {
					let mut menu_item = ViIconMenuItem::new((), value);

					init(&mut menu_item);
					menu.append(&*menu_item);
					menu_item.show();
				}
				AppTrayMenuItem::Separator => {
					let separator = gtk::SeparatorMenuItem::new();
					menu.append(&separator);
					separator.show();
				}
			}
		}

		let indicator = Indicator::new(id, icon, IndicatorCategory::ApplicationStatus);
		indicator.set_status(IndicatorStatus::Active);
		maybe!((title) {
			indicator.set_title(Some(title));
		});
		maybe!((desc) {
			indicator.set_attention_icon_full(icon, desc);
		});
		indicator.set_menu(Some(&menu));

		Self(menu)
	}

	#[inline]
	pub fn main(&self) {
		gtk::main();
	}
}
