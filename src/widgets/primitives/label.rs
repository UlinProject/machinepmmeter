use crate::maybe;
use crate::{__gen_transparent_gtk_type, config::FontConfig, core::maybe::Maybe};
use gtk::pango;
use gtk::{
	Align, Label,
	ffi::GtkLabel,
	pango::{AttrFontDesc, AttrList, FontDescription},
	traits::{LabelExt, StyleContextExt, WidgetExt},
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
	pub fn new<'c>(
		class: impl Maybe<&'c str>,
		fconfig: impl AsRef<FontConfig>,
		value: &str,
		weight: impl Maybe<pango::Weight>,
	) -> Self {
		let fconfig = fconfig.as_ref();
		let label = Label::new(Some(value));

		{
			// font
			let mut font_desc;
			let font_attr = AttrFontDesc::new({
				font_desc = FontDescription::new();
				font_desc.set_family(fconfig.get_family());
				font_desc.set_absolute_size(fconfig.calc_font_size());
				//font_desc.set_weight(pango::Weight::Normal);
				maybe!((weight) font_desc.set_weight(weight));
				&font_desc
			});
			let attrs = AttrList::new();
			attrs.insert(font_attr);
			label.set_attributes(Some(&attrs));
		}
		{
			let style = label.style_context();
			style.add_class("vilabel");
			maybe!((class) style.add_class(class));
		}

		Self(label)
	}

	pub fn set_text(&self, text: &str) {
		self.0.set_text(text);
	}

	pub fn set_align(self, align: Align) -> Self {
		self.0.set_halign(align);
		self.0.set_valign(align);

		self
	}

	#[inline]
	pub fn set_margin(self, margin: i32) -> Self {
		self.0.set_margin(margin);

		self
	}

	#[inline]
	pub fn set_margin_top(self, margin: i32) -> Self {
		self.0.set_margin_top(margin);

		self
	}

	#[inline]
	pub fn set_margin_start(self, margin: i32) -> Self {
		self.0.set_margin_start(margin);

		self
	}

	#[inline]
	pub fn set_margin_end(self, margin: i32) -> Self {
		self.0.set_margin_end(margin);

		self
	}

	#[inline]
	pub fn set_margin_bottom(self, margin: i32) -> Self {
		self.0.set_margin_bottom(margin);

		self
	}

	pub fn connect_nonblack_background(self, red: f64, green: f64, blue: f64, alpha: f64) -> Self {
		if red != 0.0 || green != 0.0 || blue != 0.0 || alpha != 1.0 {
			return self.connect_background(red, green, blue, alpha);
		}
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
