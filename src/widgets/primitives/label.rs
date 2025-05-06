use crate::app::config::FontAppConfig;
use crate::maybe;
use crate::{__gen_transparent_gtk_type, core::maybe::Maybe};
use gtk::pango;
use gtk::{
	Align, Label,
	ffi::GtkLabel,
	pango::{AttrFontDesc, AttrList, FontDescription},
	traits::{LabelExt, StyleContextExt, WidgetExt},
};
pub use pango::Weight;
pub use pango::WrapMode;

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
		f_app_config: impl AsRef<FontAppConfig>,
		value: &str,
		weight: impl Maybe<Weight>,
	) -> Self {
		let f_app_config = f_app_config.as_ref();
		let label = Label::new(Some(value));
		{
			let style = label.style_context();
			style.add_class("vilabel");
			maybe!((class) style.add_class(class));
		}
		
		{
			// font
			let mut font_desc;
			let font_attr = AttrFontDesc::new({
				font_desc = FontDescription::new();
				font_desc.set_family(f_app_config.get_family());
				font_desc.set_absolute_size(f_app_config.calc_font_size());
				//font_desc.set_weight(pango::Weight::Normal);
				maybe!((weight) font_desc.set_weight(weight));
				&font_desc
			});
			let attrs = AttrList::new();
			attrs.insert(font_attr);
			label.set_attributes(Some(&attrs));
		}
		label.set_visible(true);

		Self(label)
	}

	#[inline]
	pub fn set_text(&self, text: &str) {
		self.0.set_text(text);
	}

	#[inline]
	pub fn read_text<R>(&self, next: impl FnOnce(&str) -> R) -> R {
		let text = self.0.text();

		next(text.as_str())
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
	pub fn set_wrap(self, wrap: bool) -> Self {
		self.0.set_wrap(wrap);

		self
	}

	#[inline]
	pub fn set_wrap_mode(self, wrap_mode: WrapMode) -> Self {
		self.0.set_wrap_mode(wrap_mode);

		self
	}

	#[inline]
	pub fn set_max_width_chars(self, width: i32) -> Self {
		self.0.set_max_width_chars(width);

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

	#[inline]
	pub fn set_visible(self, v: bool) -> Self {
		self.0.set_visible(v);

		self
	}

	#[inline]
	pub fn set_visible2(&self, v: bool) {
		self.0.set_visible(v);
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
