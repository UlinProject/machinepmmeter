use crate::__gen_transparent_gtk_type;
use crate::app::config::FontAppConfig;
use crate::widgets::primitives::label::ViLabel;
use gtk::Align;
use gtk::Box;
use gtk::Image;
use gtk::Orientation;
use gtk::ffi::GtkBox;
use gtk::traits::BoxExt;
use gtk::traits::WidgetExt;

#[repr(transparent)]
#[derive(Debug)]
pub struct ViHotkeyItem(Box);

__gen_transparent_gtk_type! {
	#[sys(GtkBox)]
	ViHotkeyItem(
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

impl ViHotkeyItem {
	pub fn new(f_app_config: impl AsRef<FontAppConfig>, icon: &str, text: &str) -> Self {
		let hbox = Box::new(Orientation::Horizontal, 0);

		let image = Image::from_icon_name(Some(icon), gtk::IconSize::Button);
		image.set_margin_start(4);
		hbox.pack_start(&image, false, false, 0);

		let label = ViLabel::new((), f_app_config, text, ())
			.set_align(Align::Start)
			.set_margin_top(4)
			.set_margin_start(3);
		hbox.pack_start(&label, false, false, 0);
		hbox.set_visible(true);

		Self(hbox)
	}
}
