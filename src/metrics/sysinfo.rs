use crate::app::config::AppConfig;
use crate::app::consts::UPPERCASE_APP_PKG_NAME;
use crate::app::consts::UPPERCASE_APP_PKG_VERSION;
use crate::widgets::notebook::ViNotebook;
use crate::widgets::primitives::label::ViLabel;
use gtk::Align;
use gtk::pango::Weight;
use gtk::pango::WrapMode;
use gtk::traits::BoxExt;
use gtk::traits::StyleContextExt;
use gtk::traits::WidgetExt;
use std::rc::Rc;
use sys_metrics::host::get_hostname;
use sys_metrics::host::get_kernel_version;
use sys_metrics::host::get_os_version;

pub fn vinotebook_append_page(app_config: &Rc<AppConfig>, vinotebook: &ViNotebook) {
	let vbox = vinotebook.append_page(
		&**app_config,
		"?",
		get_hostname()
			.ok()
			.map(|hn| format!("Hostname: {}", hn))
			.as_deref(),
	);
	{
		let style = vbox.style_context();
		style.add_class("viboxsysinfo");
	}
	vbox.connect_draw(move |da, cr| {
		let (width, height) = {
			let allocation = da.allocation();

			(allocation.width(), allocation.height())
		};
		let (width, height) = (width.into(), height.into());
		{
			// background
			let (r, g, b, a) = (0.0, 0.0, 0.0, 0.6);

			cr.set_source_rgba(r, g, b, a);
			cr.rectangle(0.0, 0.0, width, height);
			let _e = cr.fill();
		}

		false.into()
	});

	vbox.pack_start(
		&ViLabel::new("head0", &**app_config, UPPERCASE_APP_PKG_NAME, Weight::Bold)
			.set_margin_top(24)
			.set_margin_bottom(0)
			.set_align(Align::Center),
		true,
		true,
		0,
	);

	vbox.pack_start(
		&ViLabel::new("head1", &**app_config, UPPERCASE_APP_PKG_VERSION, ())
			.set_margin_bottom(18)
			.set_align(Align::Center),
		true,
		true,
		0,
	);

	if let Ok(kversion) = get_kernel_version() {
		vbox.pack_start(
			&ViLabel::new("head", &**app_config, "Kernel version: ", Weight::Bold)
				.set_margin_top(4)
				.set_margin_start(6)
				.set_align(Align::Start),
			true,
			true,
			0,
		);
		vbox.pack_start(
			&ViLabel::new("value", &**app_config, &kversion, ())
				.set_margin_start(6)
				.set_margin_end(6)
				.set_margin_bottom(4)
				.set_align(Align::Start)
				.set_wrap(true)
				.set_wrap_mode(WrapMode::Word)
				.set_max_width_chars(45),
			true,
			true,
			0,
		);
	}
	if let Ok(os) = get_os_version() {
		vbox.pack_start(
			&ViLabel::new("head", &**app_config, "Operating system: ", Weight::Bold)
				.set_margin_top(4)
				.set_margin_start(6)
				.set_align(Align::Start),
			true,
			true,
			0,
		);
		vbox.pack_start(
			&ViLabel::new("value", &**app_config, &os, ())
				.set_margin_start(6)
				.set_margin_end(6)
				.set_margin_bottom(24)
				.set_align(Align::Start)
				.set_wrap(true)
				.set_wrap_mode(WrapMode::Word)
				.set_max_width_chars(45),
			true,
			true,
			0,
		);
	}
}
