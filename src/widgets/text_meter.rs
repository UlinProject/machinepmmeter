use crate::{
	__gen_transparent_gtk_type,
	app::config::{ColorAppConfig, FontAppConfig},
	widgets::primitives::{color_block::ViColorBlock, label::ViLabel},
};
use gtk::{
	Align, Box, Orientation,
	ffi::GtkBox,
	pango::Weight,
	traits::{BoxExt, StyleContextExt, WidgetExt},
};
use std::{cell::RefCell, ops::Deref, rc::Rc};

#[repr(transparent)]
#[derive(Debug)]
pub struct ViTextMeter(Box);

__gen_transparent_gtk_type! {
	#[sys(GtkBox)]
	ViTextMeter(
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

impl ViTextMeter {
	pub fn new_sender(
		app_config: impl AsRef<FontAppConfig> + AsRef<ColorAppConfig> + Copy,

		transparent: f64,
	) -> ViTextMeterSender {
		let hbox = Box::new(Orientation::Horizontal, 0);
		hbox.style_context().add_class("vitextmeter");
		hbox.connect_draw(move |window, cr| {
			let allocation = window.allocation();
			cr.set_source_rgba(0.220, 0.220, 0.220, transparent);

			cr.rectangle(
				0.0,
				0.0,
				allocation.width().into(),
				allocation.height().into(),
			);
			let _e = cr.fill();

			false.into()
		});
		let margin_top = 3 + 3;
		let margin_bottom = 3;

		let color = {
			let (red, green, blue, transparent) = (app_config.as_ref() as &ColorAppConfig)
				.green()
				.into_rgba(transparent);

			let state_color = Rc::new(RefCell::new((red, green, blue, transparent)));
			let color_block =
				ViColorBlock::new(2, 0).connect_state_background::<true>(state_color.clone());

			hbox.pack_start(&color_block, false, true, 0);

			(state_color, color_block)
		};

		let current = ViLabel::new("arg_vitextmeter", app_config, "0.0", Weight::Ultrabold)
			.set_align(Align::Start)
			.set_margin_start(10)
			.set_margin_top(margin_top)
			.set_margin_bottom(margin_bottom);
		hbox.pack_start(&current, false, true, 0);

		let avg = ViLabel::new("arg_vitextmeter", app_config, "AVG: 0", Weight::Normal)
			.set_visible(false)
			.set_align(Align::Center)
			.set_margin_top(margin_top)
			.set_margin_bottom(margin_bottom);
		hbox.pack_start(&avg, true, true, 0);

		let limit = ViLabel::new("arg_vitextmeter", app_config, "LIMIT: 0", Weight::Normal)
			.set_visible(false)
			.set_margin_end(8)
			.set_align(Align::End)
			.set_margin_top(margin_top)
			.set_margin_bottom(margin_bottom);
		hbox.pack_start(&limit, true, true, 0);

		hbox.set_visible(true);

		ViTextMeterSender {
			color,
			current,
			avg,
			limit,
			gui: Self(hbox),
		}
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

	#[inline]
	pub fn set_margin_bottom2(&self, margin: i32) {
		self.0.set_margin_bottom(margin);
	}
}

pub struct ViTextMeterSender {
	#[allow(clippy::type_complexity)]
	color: (Rc<RefCell<(f64, f64, f64, f64)>>, ViColorBlock),
	current: ViLabel,
	avg: ViLabel,
	limit: ViLabel,

	gui: ViTextMeter,
}

impl ViTextMeterSender {
	pub fn set_colordata<R>(&self, next: impl FnOnce(&mut (f64, f64, f64, f64)) -> R) -> R {
		let mut write = RefCell::borrow_mut(&self.color.0);

		next(&mut write)
	}

	pub fn set_color(&self, r: f64, g: f64, b: f64) {
		self.set_colordata(|w| {
			w.0 = r;
			w.1 = g;
			w.2 = b;
		});
	}

	pub fn set_color_and_queue_draw(&self, r: f64, g: f64, b: f64) {
		self.set_color(r, g, b);
		self.color.1.queue_draw();
	}

	pub fn set_current_and_queue_draw(&self, v: &str) {
		let v = v.get(..6).map_or(v, |v| v);

		self.current.set_text(v);
	}

	pub fn set_avg_and_queue_draw(&self, v: &str) {
		let v = v.get(..6).map_or(v, |v| v);

		self.avg.set_text(&format!("AVG: {v}")); // TODO REFACTORING ME
	}

	pub fn set_limit_and_queue_draw(&self, v: &str) {
		let v = v.get(..6).map_or(v, |v| v);

		self.limit.set_text(&format!("LIMIT: {v}")); // TODO REFACTORING ME
	}

	pub fn is_visible_limit(&self) -> bool {
		self.limit.is_visible()
	}

	pub fn is_visible_avg(&self) -> bool {
		self.avg.is_visible()
	}

	pub fn set_visible_limit(&self, v: bool) {
		self.limit.set_visible2(v);
	}

	pub fn set_visible_avg(&self, v: bool) {
		self.avg.set_visible2(v);
	}
}

impl Deref for ViTextMeterSender {
	type Target = ViTextMeter;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.gui
	}
}
