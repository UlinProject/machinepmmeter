use crate::app::config::AppConfig;
use crate::widgets::ViMeter;
use crate::widgets::notebook::ViNotebook;
use crate::widgets::primitives::graph::ViGraphBackgroundSurface;
use glib::ControlFlow;
use gtk::traits::BoxExt;
use std::cell::RefCell;
use std::rc::Rc;

pub fn vinotebook_append_page(
	app_config: &Rc<AppConfig>,
	vigraph_surface: &ViGraphBackgroundSurface,
	width: i32,
	vinotebook: &ViNotebook,
) {
	let vbox = vinotebook.append_page(
		&**app_config,
		"demo",
		Some(
			"Notice: This page does not contain any useful information and is for debugging purposes only.",
		),
	);
	{
		let vimetr = ViMeter::new_visender(
			app_config.clone(),
			"# Demo (time: 80, value: 0.7)",
			width,
			200,
			Some(vigraph_surface.clone()),
			1.0,
		);
		vbox.pack_start(&*vimetr, false, false, 0);
		glib::timeout_add_local(std::time::Duration::from_millis(80), move || {
			vimetr.push_next_and_queue_draw(0.7, 0.7, 1.0, 0.0, 0.0);

			ControlFlow::Continue
		});
	}
	{
		let vimetr = ViMeter::new_visender(
			app_config.clone(),
			"# Demo (time: 10ms, step: 0.1)",
			width,
			200,
			Some(vigraph_surface.clone()),
			1.0,
		);
		let data = RefCell::new(0.0);
		vbox.pack_start(&*vimetr, false, false, 0);
		glib::timeout_add_local(std::time::Duration::from_millis(10), move || {
			let mut w = RefCell::borrow_mut(&data);
			vimetr.push_next_and_queue_draw(*w, *w, 1.0, 0.0, 0.0);

			*w += 0.1;
			if *w >= 1.0 {
				*w = 0.0;
			}

			ControlFlow::Continue
		});
	}
	{
		let vimetr = ViMeter::new_visender(
			app_config.clone(),
			"# Demo (time: 1ms, step: 0.01)",
			width,
			200,
			Some(vigraph_surface.clone()),
			1.0,
		);
		let data = RefCell::new(0.0);
		vbox.pack_start(&*vimetr, false, false, 0);
		glib::timeout_add_local(std::time::Duration::from_millis(1), move || {
			let mut w = RefCell::borrow_mut(&data);
			vimetr.push_next_and_queue_draw(*w, *w, 1.0, 0.0, 0.0);

			*w += 0.01;
			if *w >= 1.0 {
				*w = 0.0;
			}

			ControlFlow::Continue
		});
	}
	{
		let vimetr = ViMeter::new_visender(
			app_config.clone(),
			"# Demo (time: 1ms, step: 0.001)",
			width,
			200,
			Some(vigraph_surface.clone()),
			1.0,
		);
		let data = RefCell::new(0.0);
		vbox.pack_start(&*vimetr, false, false, 0);
		glib::timeout_add_local(std::time::Duration::from_millis(1), move || {
			let mut w = RefCell::borrow_mut(&data);
			vimetr.push_next_and_queue_draw(*w, *w, 1.0, 0.0, 0.0);

			*w += 0.001;
			if *w >= 1.0 {
				*w = 0.0;
			}

			ControlFlow::Continue
		});
	}
}
