use crate::__gen_transparent_gtk_type;
use crate::config::Config;
use crate::core::maybe::Maybe;
use crate::maybe;
use crate::widgets::primitives::graph::ViGraph;
use crate::widgets::primitives::graph::ViGraphSender;
use crate::widgets::primitives::label::ViLabel;
use crate::widgets::text_meter::ViTextMeter;
use crate::widgets::text_meter::ViTextMeterSender;
use gtk::Align;
use gtk::Box;
use gtk::ffi::GtkBox;
use gtk::traits::BoxExt;
use std::ops::Deref;
use std::rc::Rc;

pub mod primitives {
	pub mod color_block;
	pub mod graph;
	pub mod label;
}

pub mod dock_head;
pub mod text_meter;

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
		config: Rc<Config>,
		head: impl Maybe<&'a str>,
		width: i32,
		len: usize,
		transparent: f64,
	) -> ViMeterSender {
		let vbox = Box::new(gtk::Orientation::Vertical, 0);

		maybe!((head)
			vbox.pack_start(
				&ViLabel::new("info_vitextmeter", &*config, head, ())
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

		let textmeter_sender = ViTextMeter::new_sender(&*config, transparent);
		vbox.pack_start(&*textmeter_sender, false, false, 0);

		let graphsender = ViGraph::new_graphsender(config.clone(), width, 42, len, transparent);
		vbox.pack_start(&*graphsender, true, true, 0);

		ViMeterSender {
			config,
			color_and_text: textmeter_sender,
			graph: graphsender,
			meter: Self(vbox),
		}
	}
}

#[allow(dead_code)]
pub struct ViMeterSender {
	config: Rc<Config>,
	color_and_text: ViTextMeterSender,
	graph: ViGraphSender,
	meter: ViMeter,
}

impl ViMeterSender {
	pub fn push_next_and_queue_draw(&self, v: f64) {
		self.graph.push_next(v);

		let color = self.config.get_color_config();
		let (red, green, blue) = if v >= 0.85 {
			color.red()
		} else if v >= 0.75 {
			color.orange()
		} else {
			color.green()
		};
		let red = (red as f64) / 255.0;
		let green = (green as f64) / 255.0;
		let blue = (blue as f64) / 255.0;
		self.color_and_text
			.set_color_and_queue_draw(red, green, blue);

		self.color_and_text
			.set_current_and_queue_draw(&v.to_string()); // TODO REFACTOING ME
		self.color_and_text.set_avg_and_queue_draw(&v.to_string()); // TODO REFACTOING ME
		self.color_and_text.set_limit_and_queue_draw(&v.to_string()); // TODO REFACTOING ME

		self.graph.queue_draw();
	}
}

impl Deref for ViMeterSender {
	type Target = ViMeter;

	fn deref(&self) -> &Self::Target {
		&self.meter
	}
}
