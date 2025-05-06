use crate::app::config::AppConfig;
use crate::core::f64sbuff::F64SBuff;
use crate::core::maybe::Maybe;
use crate::widgets::ViMeter;
use crate::widgets::notebook::ViNotebook;
use crate::widgets::primitives::graph::background::ViGraphBackgroundSurface;
use crate::widgets::primitives::graph::stream::ViGraphArcSyncStream;
use crate::widgets::primitives::graph::stream::ViGraphStream;
use crate::widgets::primitives::label::ViLabel;
use async_channel::Receiver;
use dbus_udisks2::Disks;
use dbus_udisks2::UDisks2;
use dbus_udisks2::smart::SmartValue;
use enclose::enc;
use gtk::Align;
use gtk::Box;
use gtk::pango::Weight;
use gtk::traits::BoxExt;
use gtk::traits::WidgetExt;
use log::error;
use log::trace;
use std::num::NonZeroUsize;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::OnceLock;
use std::time::Duration;

#[repr(transparent)]
#[derive(Debug)]
struct OnceWaitResult<T>(Arc<_WaitResult<T>>);

impl<T> Clone for OnceWaitResult<T> {
	#[inline]
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<T> OnceWaitResult<T> {
	pub fn new() -> Self {
		Self(Arc::new(_WaitResult {
			wait: Barrier::new(2),
			data: OnceLock::new(),
		}))
	}

	pub fn set_and_waitend(self, data: T) -> Result<(), T> {
		let result = self.0.data.set(data);

		self.wait_and_end();
		result
	}

	#[inline]
	pub fn wait_and_end(self) {
		let _e = self.0.wait.wait();

		drop(self);
	}

	pub fn wait_endresult<R>(self, mut next: impl FnMut(&T) -> Option<R>) -> Option<R> {
		let _e = self.0.wait.wait();

		if let Some(v) = self.0.data.get() {
			return next(v);
		}
		None
	}
}

#[derive(Debug)]
struct _WaitResult<T> {
	wait: Barrier,
	data: OnceLock<T>,
}

struct U2Item {
	model_info: ModelInfo,
	stream: ViGraphArcSyncStream,
	recv: Receiver<U2Events>,
}

enum U2Events {
	QueueDraw(f64, f64),
}

struct ModelInfo {
	name: String,
	serial: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
enum SensorType {
	#[default]
	Unknown,
	Temperature,
	Power,
	Current,
}

#[allow(clippy::too_many_arguments)]
pub fn vinotebook_append_page(
	app_config: &Rc<AppConfig>,
	vigraph_surface: &ViGraphBackgroundSurface,
	width: impl Maybe<i32> + Copy,
	height: impl Maybe<i32> + Copy,
	len: usize,
	complete_redraw_step_time: Duration, // graph + limit + current

	graph_count_elements_on_onestep: NonZeroUsize,
	graph_time_onestep: Duration,

	vinotebook: &ViNotebook,
) {
	let waitinitlist: OnceWaitResult<Vec<U2Item>> = OnceWaitResult::new();
	std::thread::spawn(enc!((waitinitlist) move || {
		let mut exp_init_sensors = Vec::with_capacity(12);

		let mut a_sensors = Vec::with_capacity(12);
		trace!("udisks2:");
		if let Ok(udisks2) = UDisks2::new() {
			let disks = Disks::new(&udisks2);
			for device in disks.devices {
				trace!("	{}(model), {}(serial)", device.drive.model, device.drive.serial);

				if let Ok(SmartValue::Enabled(smart_data)) = udisks2.smart_attributes(&device.drive, true) {
					let model = device.drive.model.clone();
					let serial = device.drive.serial.clone();

					let model_info = ModelInfo {
						name: model,
						serial,
					};

					//println!("{:?}", smart_data.attributes);
					let c_value = smart_data.temperature - 273.15;
					trace!("		{}", c_value);

					/*let min = 0.0;
					let max = 100.0;

					for a in smart_data.attributes {
						if a.name == "airflow-temperature-celsius" {
							println!("{:?}", a);

							break;
						}
					}*/

					let stream = ViGraphArcSyncStream::with_len(len);
					let (sender, recv) = async_channel::bounded(32);
					exp_init_sensors.push(U2Item {
						model_info,
						stream: stream.clone(),
						recv,
					});

					a_sensors.push((device.drive, stream, sender));
				}
			}

			if let Err(_exp_init_sensors) = waitinitlist.set_and_waitend(exp_init_sensors) {
				error!("#[udisks2, send] Feedback is broken, i can't continue initialization.");

				return;
			}
			if !a_sensors.is_empty() {
				loop {
					for (device, stream, sender) in &a_sensors {
						let min = 0.0;
						let max = 100.0;
						let mut exp_elements = graph_count_elements_on_onestep.get();
						let mut current = 0.0;
						loop {
							if let Ok(SmartValue::Enabled(smart_data)) = udisks2.smart_attributes(device, true) {
								current = smart_data.temperature - 273.15;

								let a = (current - min) / (max - min);
								stream.write(|stream| {
									stream.push_next(a);
								});
							}

							exp_elements -= 1;
							if exp_elements == 0 {
								break;
							}
							std::thread::sleep(graph_time_onestep);
						}

						let _e = sender.send_blocking(U2Events::QueueDraw(current, max));
					}

					std::thread::sleep(complete_redraw_step_time);
				}
			}
		}
	}));

	if waitinitlist.wait_endresult(|exp_init_sensors| {
		if exp_init_sensors.is_empty() {
			error!("#[udisks2, recv] No sensors were found in the system, there is nothing to do on this platform.");

			return Some(());
		}

		let rvbox = vinotebook.append_page(
			&**app_config,
			"udisks2",
			None,
		);
		for item in exp_init_sensors {
			let vbox = Box::new(gtk::Orientation::Horizontal, 0);
			vbox.set_valign(gtk::Align::Baseline);
			vbox.set_halign(gtk::Align::Fill);

			vbox.set_visible(true);

			vbox.pack_start(
				&ViLabel::new(
					"info_vitextmeter",
					&**app_config,
					"#",
					Weight::Bold,
				)
				.set_margin_top(4)
				.set_margin_start(4)
				.set_margin_bottom(2)
				.set_align(Align::Start),
				false,
				false,
				0,
			);

			vbox.pack_start(
				&ViLabel::new(
					"info_vitextmeter",
					&**app_config,
					&item.model_info.name,
					Weight::Bold,
				)
				.set_margin_top(4)
				.set_margin_start(4)
				.set_margin_bottom(2)
				.set_align(Align::Start),
				false,
				false,
				0,
			);

			rvbox.pack_start(&vbox, false,
				false,
				0,);

			let vimetr = ViMeter::new_visender(
				app_config.clone(),
				item.model_info.serial.as_str(),
				width,
				height,
				item.stream.clone(),
				Some(vigraph_surface.clone()),
				1.0,
			);
			vimetr.set_visible_graph(true);
			rvbox.pack_start(&*vimetr, false, false, 0);

			glib::MainContext::default().spawn_local(
				enc!((item.recv => item) async move {
					let mut f64sbuff = F64SBuff::new();

					let mut old_current = Default::default();
					let mut old_max = Default::default();
					while let Ok(event) = item.recv().await {
						match event {
							U2Events::QueueDraw(current, max) => {
								if current != old_current {
									vimetr.set_current_and_queue_draw(&f64sbuff.format_and_get(current));
									old_current = current;
								}
								if max != old_max {
									vimetr.set_limit_and_queue_draw(&f64sbuff.format_and_get(max));
									old_max = max;
								}

								vimetr.queue_draw();
							},
						}
					}
				}
			));
		}

		Some(())
	}).is_none() {
		error!("#[lm_sensors, recv] Feedback is broken, i can't continue initialization.");
	}
}
