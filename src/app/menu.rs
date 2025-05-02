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
pub struct AppMenu(Menu);

__gen_transparent_gtk_type! {
	#[sys(GtkMenu)]
	AppMenu(
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

impl Deref for AppMenu {
	type Target = Menu;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub enum AppMenuItem<'i, 'v, F>
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

impl<'i, 'v, F> AppMenuItem<'i, 'v, F>
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

impl AppMenu {
	pub fn new<'i, 'v, 'desc, 'title, F: FnMut(&'_ mut ViIconMenuItem)>(
		id: &'_ str,
		icon: &'_ str,
		title: impl Maybe<&'title str>,
		desc: impl Maybe<&'desc str>,
		items: impl Iterator<Item = AppMenuItem<'i, 'v, F>>,
	) -> Self {
		let menu = gtk::Menu::new();
		for item in items {
			match item {
				AppMenuItem::IconItem {
					icon,
					value,
					mut init,
				} => {
					let mut menu_item = ViIconMenuItem::new(icon, value);

					init(&mut menu_item);
					menu.append(&*menu_item);
					menu_item.show();
				}
				AppMenuItem::Item { value, mut init } => {
					let mut menu_item = ViIconMenuItem::new((), value);

					init(&mut menu_item);
					menu.append(&*menu_item);
					menu_item.show();
				}
				AppMenuItem::Separator => {
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
