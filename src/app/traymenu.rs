use crate::APP_ID;
use crate::PKG_DESCRIPTION;
use crate::PKG_ICON;
use crate::PKG_NAME;
use crate::app::events::AppEventSender;
use crate::core::traymenu::TrayMenu;
use crate::core::traymenu::TrayMenuItem;
use crate::widgets::primitives::iconmenuitem::ViIconMenuItem;
pub use appindicator3::IndicatorCategory;
use enclose::enc;
use gtk::traits::GtkMenuItemExt;

pub fn app_traymenu(tx_appevents: &AppEventSender) -> TrayMenu {
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

	let tray_menu = TrayMenu::new(
		APP_ID,
		PKG_ICON,
		PKG_NAME,
		PKG_DESCRIPTION,
		[
			TrayMenuItem::icon_item("view-conceal-symbolic", "Hide | Show", hide_or_show),
			TrayMenuItem::Separator,
			TrayMenuItem::icon_item("go-next-symbolic", "Next tab", next_tab),
			TrayMenuItem::icon_item("go-previous-symbolic", "Previous tab", prev_tab),
			TrayMenuItem::Separator,
			TrayMenuItem::icon_item(
				"sidebar-show-right-symbolic-rtl",
				"Next position",
				next_position,
			),
			TrayMenuItem::Separator,
			TrayMenuItem::item("About the program", abouttheprogram),
			TrayMenuItem::icon_item("system-shutdown-symbolic", "Exit", exit),
		]
		.into_iter(),
	);

	tray_menu
}
