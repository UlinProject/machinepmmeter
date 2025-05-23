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
use enclose::enc;
use gtk::Align;
use gtk::Box;
use gtk::pango::Weight;
use gtk::traits::BoxExt;
use gtk::traits::WidgetExt;
use lm_sensors::SubFeatureRef;
use lm_sensors::Value;
use lm_sensors::value::Unit;
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

struct LmItem {
	chip_info: Option<ChipInfo>,
	feature_name: String,
	stream: ViGraphArcSyncStream,
	recv: Receiver<LmEvents>,
}

enum LmEvents {
	QueueDraw(f64, f64),
}

struct ChipInfo {
	name: String,
	bus: String,
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
	let waitinitlist: OnceWaitResult<(Vec<LmItem>, Option<String>)> = OnceWaitResult::new();
	std::thread::spawn(enc!((waitinitlist) move || {
		let mut exp_init_sensors = Vec::with_capacity(12);

		let mut a_sensors = Vec::with_capacity(12);
		trace!("lm_sensors:");
		if let Ok(lmsensors) = lm_sensors::Initializer::default().initialize() {
			for chip in lmsensors.chip_iter(None) {
				if let Ok(chip_name) = chip.name() {
					trace!("{} (chip_name):", chip_name);
					let mut chip_info = Some(ChipInfo {
						name: chip_name,
						bus: chip.bus().to_string(),
					});

					for feature in chip.feature_iter() {
						if let Some(Ok(feature_name)) = feature.name() {
							trace!("	{}(feature_name): {}(feature)", feature_name, feature);


							#[derive(Debug, Clone, Default)]
							struct LmSensor<'a> {
								input: Option<(f64, Unit, SubFeatureRef<'a>)>,
								max: Option<(f64,Unit, SubFeatureRef<'a>)>,
								min: Option<(f64, Unit, SubFeatureRef<'a>)>,
								crit: Option<(f64, Unit, SubFeatureRef<'a>)>,
								r#type: SensorType,
							}

							let mut c_value = LmSensor::default();
							for sub_feature in feature.sub_feature_iter() {
								if let Some(Ok(name)) = sub_feature.name() {
									trace!("		{}(name):", name);

									if let Ok(value) = sub_feature.value() {
										match value {
											/*Value::VoltageInput(_) => {},
											Value::VoltageMinimum(_) => {},
											Value::VoltageMaximum(_) => {},
											Value::VoltageLCritical(_) => {},
											Value::VoltageCritical(_) => {},
											Value::VoltageAverage(_) => {},
											Value::VoltageLowest(_) => {},
											Value::VoltageHighest(_) => {},*/
											/*Value::VoltageAlarm(_) => {},
											Value::VoltageMinimumAlarm(_) => {},
											Value::VoltageMaximumAlarm(_) => {},
											Value::VoltageBeep(_) => {},
											Value::VoltageLCriticalAlarm(_) => {},
											Value::VoltageCriticalAlarm(_) => {},*/

											/*Value::FanInput(_) => {},
											Value::FanMinimum(_) => {},
											Value::FanMaximum(_) => {},
											Value::FanAlarm(_) => {},
											Value::FanFault(_) => {},
											Value::FanDivisor(_) => {},
											Value::FanBeep(_) => {},
											Value::FanPulses(_) => {},
											Value::FanMinimumAlarm(_) => {},
											Value::FanMaximumAlarm(_) => {},*/

											Value::TemperatureInput(a) => {
												c_value.input = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Temperature;
											},
											Value::TemperatureMaximum(a) => {
												c_value.max = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Temperature;
											},
											Value::TemperatureMaximumHysteresis(_) => {},
											Value::TemperatureMinimum(a) => {
												c_value.min = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Temperature;
											},
											Value::TemperatureCritical(a) => {
												c_value.crit = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Temperature;
											},
											Value::TemperatureCriticalHysteresis(_) => {},
											Value::TemperatureLCritical(_) => {},
											Value::TemperatureEmergency(_) => {},
											Value::TemperatureEmergencyHysteresis(_) => {},
											Value::TemperatureLowest(_) => {},
											Value::TemperatureHighest(_) => {},
											Value::TemperatureMinimumHysteresis(_) => {},
											Value::TemperatureLCriticalHysteresis(_) => {},
											Value::TemperatureAlarm(_) => {},
											Value::TemperatureMaximumAlarm(_) => {},
											Value::TemperatureMinimumAlarm(_) => {},
											Value::TemperatureCriticalAlarm(_) => {},
											Value::TemperatureFault(_) => {},
											//Value::TemperatureType(a) => {},
											Value::TemperatureOffset(_) => {},
											/*Value::TemperatureBeep(_) => {},
											Value::TemperatureEmergencyAlarm(_) => {},
											Value::TemperatureLCriticalAlarm(_) => {},*/

											/*Value::PowerAverage(_) => {},
											Value::PowerAverageHighest(_) => {},
											Value::PowerAverageLowest(_) => {},*/
											Value::PowerInput(a) => {
												c_value.input = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Power;
											},
											/*Value::PowerInputHighest(_) => {},
											Value::PowerInputLowest(_) => {},
											Value::PowerCap(_) => {},
											Value::PowerCapHysteresis(_) => {},*/
											Value::PowerMaximum(a) => {
												c_value.max = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Power;
											},
											Value::PowerCritical(a) => {
												c_value.crit = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Power;
											},
											Value::PowerMinimum(a) => {
												c_value.min = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Power;
											},
											/*Value::PowerLCritical(_) => {},
											Value::PowerAverageInterval(_) => {},
											Value::PowerAlarm(_) => {},
											Value::PowerCapAlarm(_) => {},
											Value::PowerMaximumAlarm(_) => {},
											Value::PowerCriticalAlarm(_) => {},
											Value::PowerMinimumAlarm(_) => {},
											Value::PowerLCriticalAlarm(_) => {},*/

											// Value::EnergyInput(_) => {},

											Value::CurrentInput(a) => {
												c_value.input = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Current;
											},
											Value::CurrentMinimum(a) => {
												c_value.min = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Current;
											},
											Value::CurrentMaximum(a) => {
												c_value.max = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Current;
											},
											Value::CurrentLCritical(_) => {},
											Value::CurrentCritical(a) => {
												c_value.crit = Some((a, value.unit(), sub_feature));
												c_value.r#type = SensorType::Current;
											},

											/*Value::CurrentAverage(_) => {},
											Value::CurrentLowest(_) => {},
											Value::CurrentHighest(_) => {},
											Value::CurrentAlarm(_) => {},
											Value::CurrentMinimumAlarm(_) => {},
											Value::CurrentMaximumAlarm(_) => {},
											Value::CurrentBeep(_) => {},
											Value::CurrentLCriticalAlarm(_) => {},
											Value::CurrentCriticalAlarm(_) => {},*/

											/*Value::HumidityInput(_) => {},
											Value::VoltageID(_) => {},
											Value::IntrusionAlarm(_) => {},
											Value::IntrusionBeep(_) => {},
											Value::BeepEnable(_) => {},*/
											// Value::Unknown{kind,value} => {},
											_  =>  {},
										}
									}
								}
							}
							if c_value.input.is_some() && c_value.r#type != SensorType::Unknown {
								let stream = ViGraphArcSyncStream::with_len(len);
								let (sender, recv) = async_channel::bounded(32);
								exp_init_sensors.push(LmItem {
									chip_info: chip_info.take(),
									feature_name: feature_name.to_string(),
									stream: stream.clone(),
									recv,
								});

								a_sensors.push((c_value, stream, sender));
							}
						}
					}
				}
			}

			if let Err(_exp_init_sensors) = waitinitlist.set_and_waitend((exp_init_sensors, lmsensors
				.version()
				.map(|a| format!("lm_sensors: {}", a)))) {
				error!("#[lm_sensors, send] Feedback is broken, i can't continue initialization.");

				return;
			}

			if !a_sensors.is_empty() {
				loop {
					for (asensor, stream, sender) in &a_sensors {
						let mut min = 0.0;
						if let Some((_startv, _unit, sensor)) = asensor.min {
							if let Ok(v) = sensor.raw_value() {
								if v < 65261.0 && v > -273.0 {
									min = v;
								}
							}
						};
						let mut max = 100.0;
						if let Some((_startv, _unit, sensor)) = asensor.max {
							if let Ok(v) = sensor.raw_value() {
								if v != 0.0 && v < 65261.0 && v > -273.0 {
									max = v;
								}
							}
						}else if let Some((_startv, _unit, sensor)) = asensor.crit {
							if let Ok(v) = sensor.raw_value() {
								if v != 0.0 && v < 65261.0 && v > -273.0 {
									max = v;
								}
							}
						}
						if let Some((_startv, _unit, sensor)) = asensor.input {
							let mut exp_elements = graph_count_elements_on_onestep.get();
							let mut current = 0.0;
							loop {
								if let Ok(v) = sensor.raw_value() {
									if v != 0.0 && v < 65261.0 && v > -273.0 {
										current = v;
										let a = (v - min) / (max - min);

										stream.write(|stream| {
											stream.push_next(a);
										});
									}
								}

								exp_elements -= 1;
								if exp_elements == 0 {
									break;
								}
								std::thread::sleep(graph_time_onestep);
							}

							let _e = sender.send_blocking(LmEvents::QueueDraw(current, max));
						}
					}

					std::thread::sleep(complete_redraw_step_time);
				}
			}
		}
	}));

	if waitinitlist.wait_endresult(|(exp_init_sensors, lmversion)| {
		if exp_init_sensors.is_empty() {
			error!("#[lm_sensors, recv] No sensors were found in the system, there is nothing to do on this platform.");

			return Some(());
		}

		let rvbox = vinotebook.append_page(
			&**app_config,
			"lm_sensors",
			lmversion.as_deref(),
		);
		for item in exp_init_sensors {
			if let Some(ref chip_info) = item.chip_info {
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
						&chip_info.name,
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
						&chip_info.bus,
						Weight::Normal,
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
			}

			let vimetr = ViMeter::new_visender(
				app_config.clone(),
				item.feature_name.as_str(),
				width,
				height,
				item.stream.clone(),
				Some(vigraph_surface.clone()),
				1.0,
			);
			vimetr.set_visible_graph(true);
			vimetr.set_visible_limit(true);
			rvbox.pack_start(&*vimetr, false, false, 0);

			glib::MainContext::default().spawn_local(
				enc!((item.recv => item) async move {
					let mut f64sbuff = F64SBuff::new();

					let mut old_current = Default::default();
					let mut old_max = Default::default();
					while let Ok(event) = item.recv().await {
						match event {
							LmEvents::QueueDraw(current, max) => {
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
