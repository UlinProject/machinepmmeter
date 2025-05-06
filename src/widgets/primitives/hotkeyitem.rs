use crate::__gen_transparent_gtk_type;
use crate::app::config::FontAppConfig;
use crate::widgets::primitives::colorblock::ViColorBlock;
use crate::widgets::primitives::label::ViLabel;
use gtk::Align;
use gtk::Box;
use gtk::Image;
use gtk::Orientation;
use gtk::ffi::GtkBox;
use gtk::pango::Weight;
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
	pub fn new(
		f_app_config: impl AsRef<FontAppConfig>,
		icon: &str,
		info: &str,
		hotkey: &str,
	) -> Self {
		let hbox = Box::new(Orientation::Horizontal, 0);
		hbox.set_valign(gtk::Align::Baseline);
		hbox.set_halign(gtk::Align::Fill);

		{
			let cl = ViColorBlock::new(1, -1).connect_background((1.0, 1.0, 1.0, 0.6));
			cl.set_margin_start(14);
			hbox.pack_start(&cl, false, false, 0);

			cl.set_visible(true);
		}

		{
			let image = Image::from_icon_name(Some(icon), gtk::IconSize::Button);
			image.set_margin_start(10);
			hbox.pack_start(&image, false, false, 0);

			image.set_visible(true);
		}

		hbox.pack_start(
			&ViLabel::new((), &f_app_config, info, ())
				.set_align(Align::Start)
				.set_margin_top(4)
				.set_margin_start(8),
			false,
			false,
			0,
		);
		hbox.pack_start(
			&ViLabel::new((), f_app_config, hotkey, Weight::Bold)
				.set_align(Align::Start)
				.set_margin_top(4)
				.set_margin_start(3),
			false,
			false,
			0,
		);
		hbox.set_visible(true);

		Self(hbox)
	}
}
