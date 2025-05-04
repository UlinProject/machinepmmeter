use crate::app::events::AppEventSender;
use crate::core::maybe::Maybe;
use crate::maybe;
use crate::widgets::primitives::iconmenuitem::ViIconMenuItem;
use crate::APP_ID;
use crate::PKG_DESCRIPTION;
use crate::PKG_ICON;
use crate::PKG_NAME;
use appindicator3::Indicator;
pub use appindicator3::IndicatorCategory;
use appindicator3::IndicatorStatus;
use appindicator3::traits::AppIndicatorExt;
use enclose::enc;
use gtk::traits::GtkMenuItemExt;
use gtk::traits::MenuShellExt;
use gtk::traits::WidgetExt;

pub fn app_traymenu(tx_appevents: &AppEventSender) -> AppTrayMenu {
	// Tray menu
	let hide_or_show = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
		vi.connect_activate(enc!((tx_appevents) move |_| {
			tx_appevents.toggle_window_visibility();
		}));
	}) as &'_ mut dyn FnMut(&'_ mut ViIconMenuItem);

	let next_position = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
		vi.connect_activate(enc!((tx_appevents) move |_| {
			tx_appevents.move_window_to_next_position();
		}));
	});

	let next_tab = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
		vi.connect_activate(enc!((tx_appevents) move |_| {
			tx_appevents.move_tab_to_next_position();
		}));
	});

	let prev_tab = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
		vi.connect_activate(enc!((tx_appevents) move |_| {
			tx_appevents.move_tab_to_prev_position();
		}));
	});

	let abouttheprogram = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
		vi.connect_activate(enc!((tx_appevents) move |_| {
			tx_appevents.show_or_focus_aboutdialog();
		}));
	});

	let exit = enc!((tx_appevents) &mut move |vi: &mut ViIconMenuItem| {
		vi.connect_activate(enc!((tx_appevents) move |_| {
			tx_appevents.exit();
		}));
	});

	let tray_menu = AppTrayMenu::new(
		APP_ID,
		PKG_ICON,
		PKG_NAME,
		PKG_DESCRIPTION,
		[
			AppTrayMenuItem::icon_item("view-conceal-symbolic", "Hide | Show", hide_or_show),
			AppTrayMenuItem::Separator,
			AppTrayMenuItem::icon_item("go-next-symbolic", "Next tab", next_tab),
			AppTrayMenuItem::icon_item("go-previous-symbolic", "Previous tab", prev_tab),
			AppTrayMenuItem::Separator,
			AppTrayMenuItem::icon_item(
				"sidebar-show-right-symbolic-rtl",
				"Next position",
				next_position,
			),
			AppTrayMenuItem::Separator,
			AppTrayMenuItem::item("About the program", abouttheprogram),
			AppTrayMenuItem::icon_item("system-shutdown-symbolic", "Exit", exit),
		]
		.into_iter(),
	);

	tray_menu
}

#[repr(transparent)]
#[derive(Debug)]
pub struct AppTrayMenu(Indicator);

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
				}
				AppTrayMenuItem::Item { value, mut init } => {
					let mut menu_item = ViIconMenuItem::new((), value);

					init(&mut menu_item);
					menu.append(&*menu_item);
				}
				AppTrayMenuItem::Separator => {
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

pub struct AppTrayMenuNotInitialized;
