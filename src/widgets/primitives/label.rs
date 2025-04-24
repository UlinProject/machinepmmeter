use crate::__gen_transparent_gtk_type;
use gtk::{
	Label,
	ffi::GtkLabel,
	pango::{self, AttrFontDesc, AttrList, FontDescription},
	traits::{LabelExt, WidgetExt},
};

#[repr(transparent)]
#[derive(Debug)]
pub struct ViLabel(Label);

__gen_transparent_gtk_type! {
	#[sys(GtkLabel)]
	ViLabel(
		new |a: Label| {
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

impl ViLabel {
	pub fn new(value: &str) -> Self {
		let label = Label::new(Some(value));

		let mut font_desc = FontDescription::new();
		font_desc.set_family("Monospace");
		font_desc.set_absolute_size(12.0 * pango::SCALE as f64);
		font_desc.set_weight(pango::Weight::Bold);

		let font_attr = AttrFontDesc::new(&font_desc);
		let attrs = AttrList::new();
		attrs.insert(font_attr);
		label.set_attributes(Some(&attrs));

		Self(label)
	}

	pub fn set_margin(self, margin: i32) -> Self {
		self.0.set_margin(margin);

		self
	}

	pub fn connect_background(self, red: f64, green: f64, blue: f64, alpha: f64) -> Self {
		self.0.connect_draw(move |window, cr| {
			let allocation = window.allocation();
			cr.set_source_rgba(red, green, blue, alpha);

			cr.rectangle(
				0.0,
				0.0,
				allocation.width().into(),
				allocation.height().into(),
			);
			let _e = cr.fill();

			false.into()
		});

		self
	}
}
