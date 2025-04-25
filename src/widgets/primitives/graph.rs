use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;

use crate::__gen_transparent_gtk_type;
use crate::config::ColorConfig;
use crate::config::Config;
use enclose::enc;
use gtk::DrawingArea;
use gtk::cairo;
use gtk::ffi::GtkDrawingArea;
use gtk::traits::WidgetExt;

#[repr(transparent)]
#[derive(Debug)]
pub struct ViGraph(DrawingArea);

__gen_transparent_gtk_type! {
	#[sys(GtkDrawingArea)]
	ViGraph(
		new |a: DrawingArea| {
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

impl ViGraph {
	pub fn new_graphsender(
		config: Rc<Config>,
		width: i32,
		height: i32,
		len: usize,
		transparent: f64,
	) -> ViGraphSender {
		let rc_data = Rc::new(RefCell::new({
			let mut v = VecDeque::<f64>::new();
			for _ in 0..len {
				v.push_back(0.0);
			}

			v
		}));

		let graph_area = DrawingArea::new();
		graph_area.set_margin_bottom(6);
		graph_area.set_size_request(width, height);
		
		graph_area.connect_draw(enc!((rc_data) move |da, cr| {
			{
				let data = RefCell::borrow(&rc_data);
				draw_peak_graph(&*config, da, cr, data.iter(), data.len(), transparent);
			}

			false.into()
		}));

		ViGraphSender(rc_data, Self(graph_area))
	}
}

pub struct ViGraphSender(Rc<RefCell<VecDeque<f64>>>, ViGraph);

impl Deref for ViGraphSender {
	type Target = ViGraph;

	fn deref(&self) -> &Self::Target {
		&self.1
	}
}

impl ViGraphSender {
	pub fn push_next_and_queue_draw(&self, v: f64) {
		self.push_next(v);
		self.queue_draw();
	}
	
	pub fn push_next(&self, v: f64) {
		let mut lock = RefCell::borrow_mut(&self.0);
					
		lock.pop_front();
		lock.push_back(v);
	}
	
	#[inline]
	pub fn queue_draw(&self) {
		self.1.queue_draw();
	}
}

fn draw_peak_graph<'a>(
	color: impl AsRef<ColorConfig>,
	da: &DrawingArea,
	cr: &cairo::Context,
	iter: impl Iterator<Item = &'a f64> + Clone,
	len: usize,
	transparent: f64,
) {
	let color = color.as_ref();
	let allocation = da.allocation();
	let width = allocation.width().into();
	let height = allocation.height().into();

	{
		// background
		cr.move_to(0.0, 0.0);
		cr.set_source_rgba(0.255, 0.255, 0.255, transparent);

		cr.rectangle(0.0, 0.0, width, height);
		let _e = cr.fill();
	}

	let num_horizontal_lines = 10;
	let num_vertical_lines = 10;

	cr.set_source_rgba(0.8, 0.8, 0.8, transparent);
	cr.set_line_width(0.1);

	for i in 1..num_horizontal_lines {
		let y = height / num_horizontal_lines as f64 * i as f64;

		cr.move_to(0.0, y);
		cr.line_to(width, y);
		let _e = cr.stroke();
	}
	for i in 1..num_vertical_lines {
		let x = width / num_vertical_lines as f64 * i as f64;

		cr.move_to(x, 0.0);
		cr.line_to(x, height);
		let _e = cr.stroke();
	}
	cr.set_line_width(2.0);

	let a_max = {
		let mut max = 0.0;

		let iter = iter.clone();
		for a in iter {
			let a = *a;
			if a > max {
				max = a;
			}
		}

		max
	};

	let (r, g, b) = if a_max >= 0.85 {
		color.red()
	} else if a_max >= 0.75 {
		color.orange()
	} else {
		color.green()
	};

	cr.set_source_rgba(
		r as f64 / 255.0,
		g as f64 / 255.0,
		b as f64 / 255.0,
		transparent,
	);

	let x_step = width / (len - 1) as f64;
	let mut iter = iter;
	if let Some(a) = iter.next() {
		cr.move_to(0.0, height * (1.0 - a));
	}

	let mut i = 1;
	for a in iter {
		let x = i as f64 * x_step;
		let y = height * (1.0 - a);

		cr.line_to(x, y);

		i += 1;
	}

	let _e = cr.stroke();
}
