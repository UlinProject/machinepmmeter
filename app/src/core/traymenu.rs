use crate::core::maybe::Maybe;
use crate::maybe;
use crate::widgets::primitives::iconmenuitem::ViIconMenuItem;
use appindicator3::Indicator;
pub use appindicator3::IndicatorCategory;
use appindicator3::IndicatorStatus;
use appindicator3::traits::AppIndicatorExt;
use gtk::traits::MenuShellExt;
use gtk::traits::WidgetExt;

#[repr(transparent)]
#[derive(Debug)]
pub struct TrayMenu(Indicator);

pub enum TrayMenuItem<'i, 'v, F>
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

impl<'i, 'v, F> TrayMenuItem<'i, 'v, F>
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

impl TrayMenu {
	pub fn new<'i, 'v, 'desc, 'title, F: FnMut(&'_ mut ViIconMenuItem)>(
		id: &'_ str,
		icon: &'_ str,
		title: impl Maybe<&'title str>,
		desc: impl Maybe<&'desc str>,
		items: impl Iterator<Item = TrayMenuItem<'i, 'v, F>>,
	) -> Self {
		let menu = gtk::Menu::new();
		for item in items {
			match item {
				TrayMenuItem::IconItem {
					icon,
					value,
					mut init,
				} => {
					let mut menu_item = ViIconMenuItem::new(icon, value);

					init(&mut menu_item);
					menu.append(&*menu_item);
				}
				TrayMenuItem::Item { value, mut init } => {
					let mut menu_item = ViIconMenuItem::new((), value);

					init(&mut menu_item);
					menu.append(&*menu_item);
				}
				TrayMenuItem::Separator => {
					let separator = gtk::SeparatorMenuItem::new();
					menu.append(&separator);
					separator.set_visible(true);
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

		Self(indicator)
	}

	#[inline]
	pub fn is_connected(&self) -> bool {
		self.0.is_connected()
	}
}
