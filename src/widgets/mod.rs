use crate::__gen_transparent_gtk_type;
use crate::app::config::AppConfig;
use crate::core::maybe::Maybe;
use crate::maybe;
use crate::widgets::primitives::graph::background::ViGraphBackgroundSurface;
use crate::widgets::primitives::graph::stream::ViGraphStream;
use crate::widgets::primitives::graph::vi::ViGraph;
use crate::widgets::primitives::graph::vi::ViGraphSender;
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
	pub mod hotkeyitem;
	pub mod iconmenuitem;
	pub mod label;

	pub mod graph {
		pub mod background;
		pub mod data;
		pub mod stream;
		pub mod vi;
	}
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
	pub fn new_visender<'a, S>(
		app_config: Rc<AppConfig>,
		head: impl Maybe<&'a str>,
		width: impl Maybe<i32>,
		height: impl Maybe<i32>,
		stream: S,
		general_background_surface: Option<ViGraphBackgroundSurface>,
		transparent: f64,
	) -> ViMeterSender<S>
	where
		S: ViGraphStream,
	{
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
			stream,
			general_background_surface,
			width,
			height,
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
pub struct ViMeterSender<S>
where
	S: ViGraphStream,
{
	app_config: Rc<AppConfig>,
	color_and_text: ViTextMeterSender,
	graph: ViGraphSender<S>,
	meter: ViMeter,
}

impl<S> ViMeterSender<S>
where
	S: ViGraphStream,
{
	pub fn set_visible_graph(&self, vi: bool) -> bool {
		if self.graph.is_visible() != vi {
			self.graph.set_visible(vi);
			match vi {
				true => {
					self.color_and_text.set_margin_bottom2(0);
				}
				false => {
					self.color_and_text.set_margin_bottom2(6);
				}
			}

			return true;
		}

		false
	}

	pub fn set_visible_limit(&self, vi: bool) -> bool {
		if self.color_and_text.is_visible_limit() != vi {
			self.color_and_text.set_visible_limit(true);

			return true;
		}

		false
	}

	#[inline]
	pub fn set_current_and_queue_draw(&self, v: &str) {
		self.color_and_text.set_current_and_queue_draw(v);
	}

	#[inline]
	pub fn set_limit_and_queue_draw(&self, v: &str) {
		self.color_and_text.set_limit_and_queue_draw(v);
	}

	pub fn push_next_and_queue_draw(
		&self,
		current: f64,
		graph_v: impl Maybe<f64>,
		limit: impl Maybe<f64>,
		l_red: f64,
		l_orange: f64,
	) {
		maybe!((graph_v) {
			self.set_visible_graph(true);
			self.graph.push_next(graph_v);
		}else {
			self.set_visible_graph(false);
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
			self.set_visible_limit(true);
			self.color_and_text
					.set_limit_and_queue_draw(&limit.to_string()); // TODO REFACTOING ME
		} else {
			self.set_visible_limit(false);
		});
		self.color_and_text.set_visible_avg(false);

		ViMeterSender::queue_draw(self);
	}

	#[inline]
	pub fn queue_draw(&self) {
		self.graph.queue_draw();
	}
}

impl<S> Deref for ViMeterSender<S>
where
	S: ViGraphStream,
{
	type Target = ViMeter;

	fn deref(&self) -> &Self::Target {
		&self.meter
	}
}
