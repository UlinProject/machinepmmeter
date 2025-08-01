use crate::app::config::AppConfig;
use crate::app::events::AppEvents;
use crate::core::display::ViGraphDisplayInfo;
use async_channel::Receiver;
use enclose::enc;
use gtk::Application;
use gtk::CssProvider;
use gtk::gdk::Screen;
use gtk::gio::prelude::ApplicationExtManual;
use gtk::gio::traits::ApplicationExt;
use gtk::traits::CssProviderExt;
use std::rc::Rc;

#[repr(transparent)]
pub struct AppMain(Application);

impl AppMain {
	pub fn new(
		id: &str,
		cssdata: &[u8],
		app_config: Rc<AppConfig>,
		display: &Rc<ViGraphDisplayInfo>,
		rx_appevents: Rc<Receiver<AppEvents>>,
	) -> anyhow::Result<Self> {
		let defcss = {
			let a_css = CssProvider::new();
			a_css.load_from_data(cssdata)?;

			a_css
		};

		let app = Application::new(Some(id), Default::default());
		app.connect_activate(enc!((display, app_config, rx_appevents) move |app| {
			gtk::StyleContext::add_provider_for_screen(
				AsRef::<Screen>::as_ref(&display as &ViGraphDisplayInfo),
				&defcss,
				gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
			);

			let name_window = app_config.get_name_or_default();
			crate::build_ui(app, name_window, &app_config, &display, rx_appevents.clone());
		}));

		let sself = Self(app);
		Ok(sself)
	}

	pub fn run(&self) {
		self.0.run();
	}
}
