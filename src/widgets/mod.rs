use crate::__gen_transparent_gtk_type;
use crate::config::Config;
use crate::core::maybe::Maybe;
use crate::maybe;
use crate::widgets::primitives::graph::ViGraph;
use crate::widgets::primitives::graph::ViGraphSender;
use crate::widgets::primitives::label::ViLabel;
use crate::widgets::text_meter::ViTextMeter;
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
				&ViLabel::new("info_ViTextMeter", &*config, head, ())
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

		let textmeter = ViTextMeter::new(&*config, "90", "MAX: 90", "AVG: 90", transparent);
		vbox.pack_start(&textmeter, false, false, 0);

		let graphsender = ViGraph::new_graphsender(config, width, 42, len, transparent);
		vbox.pack_start(&*graphsender, true, true, 0);

		ViMeterSender(textmeter, graphsender, Self(vbox))
	}
}

#[allow(dead_code)]
pub struct ViMeterSender(ViTextMeter, ViGraphSender, ViMeter);

impl ViMeterSender {
	pub fn push_next_and_queue_draw(&self, v: f64) {
		self.1.push_next(v);
		self.1.queue_draw();
	}
}

impl Deref for ViMeterSender {
	type Target = ViMeter;

	fn deref(&self) -> &Self::Target {
		&self.2
	}
}
