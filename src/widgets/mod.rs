use crate::__gen_transparent_gtk_type;
use crate::app::config::AppConfig;
use crate::core::maybe::Maybe;
use crate::maybe;
use crate::widgets::primitives::graph::ViGraph;
use crate::widgets::primitives::graph::ViGraphBackgroundSurface;
use crate::widgets::primitives::graph::ViGraphSender;
use crate::widgets::primitives::label::ViLabel;
use crate::widgets::textmeter::ViTextMeter;
use crate::widgets::textmeter::ViTextMeterSender;
use gtk::Align;
use gtk::Box;
use gtk::ffi::GtkBox;
use gtk::traits::BoxExt;
use gtk::traits::WidgetExt;
use std::ops::Deref;
use std::rc::Rc;

pub mod primitives {
	pub mod colorblock;
	pub mod graph;
	pub mod hotkeyitem;
	pub mod iconmenuitem;
	pub mod label;
}

pub mod dockhead;
pub mod hotkeys;
pub mod notebook;
pub mod textmeter;

#[repr(transparent)]
#[derive(Debug)]
pub struct ViMeter(Box);

__gen_transparent_gtk_type! {
	#[sys(GtkBox)]
	ViMeter(
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

impl ViMeter {
	pub fn new_visender<'a>(
		app_config: Rc<AppConfig>,
		head: impl Maybe<&'a str>,
		width: i32,
		len: usize,
		general_background_surface: Option<ViGraphBackgroundSurface>,
		transparent: f64,
	) -> ViMeterSender {
		let vbox = Box::new(gtk::Orientation::Vertical, 0);

		maybe!((head)
			vbox.pack_start(
				&ViLabel::new("info_vitextmeter", &*app_config, head, ())
					.set_margin_top(4)
					.set_margin_start(4)
					.set_margin_bottom(3)
					.set_align(Align::Start)
					.connect_nonblack_background(0.0, 0.0, 0.0, transparent),
				true,
				true,
				0,
			)
		);

		let textmeter_sender = ViTextMeter::new_sender(&*app_config, transparent);
		textmeter_sender.set_margin_bottom2(6);
		vbox.pack_start(&*textmeter_sender, false, false, 0);

		let graphsender = ViGraph::new_graphsender(
			app_config.clone(),
			general_background_surface,
			width,
			42,
			len,
			transparent,
		);
		graphsender.set_margin_bottom(6);
		vbox.pack_start(&*graphsender, true, true, 0);
		vbox.set_visible(true);

		ViMeterSender {
			app_config,
			color_and_text: textmeter_sender,
			graph: graphsender,
			meter: Self(vbox),
		}
	}
}

#[allow(dead_code)]
pub struct ViMeterSender {
	app_config: Rc<AppConfig>,
	color_and_text: ViTextMeterSender,
	graph: ViGraphSender,
	meter: ViMeter,
}

impl ViMeterSender {
	pub fn push_next_and_queue_draw(
		&self,
		current: f64,
		graph_v: impl Maybe<f64>,
		limit: impl Maybe<f64>,
		l_red: f64,
		l_orange: f64,
	) {
		maybe!((graph_v) {
			if !self.graph.is_visible() {
				self.graph.set_visible(true);
				self.color_and_text.set_margin_bottom2(0);
			}
			self.graph.push_next(graph_v);
		}else {
			if self.graph.is_visible() {
				self.graph.set_visible(false);
				self.color_and_text.set_margin_bottom2(6);
			}
		});

		{
			let color = self.app_config.get_color_app_config();
			let (red, green, blue) = (if current >= l_red {
				color.red()
			} else if current >= l_orange {
				color.orange()
			} else {
				color.green()
			})
			.into_rgb();
			self.color_and_text
				.set_color_and_queue_draw(red, green, blue);
		}

		self.color_and_text
			.set_current_and_queue_draw(&current.to_string()); // TODO REFACTOING ME
		self.color_and_text
			.set_avg_and_queue_draw(&current.to_string()); // TODO REFACTOING ME
		maybe!((limit) {
			if !self.color_and_text.is_visible_limit() {
				self.color_and_text.set_visible_limit(true);
			}
			self.color_and_text
					.set_limit_and_queue_draw(&limit.to_string()); // TODO REFACTOING ME
		} else {
			if self.color_and_text.is_visible_limit() {
				self.color_and_text.set_visible_limit(false);
			}
		});
		self.color_and_text.set_visible_avg(false);

		self.graph.queue_draw();
	}
}

impl Deref for ViMeterSender {
	type Target = ViMeter;

	fn deref(&self) -> &Self::Target {
		&self.meter
	}
}
