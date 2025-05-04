use crate::app::config::AppConfig;
use crate::widgets::ViMeter;
use crate::widgets::notebook::ViNotebook;
use crate::widgets::primitives::graph::ViGraphBackgroundSurface;
use crate::widgets::primitives::label::ViLabel;
use gtk::Align;
use gtk::traits::BoxExt;
use lm_sensors::LMSensors;
use lm_sensors::SubFeatureRef;
use std::rc::Rc;

pub fn vinotebook_append_page(
	app_config: &Rc<AppConfig>,
	vigraph_surface: &ViGraphBackgroundSurface,
	width: i32,
	vinotebook: &ViNotebook,
) {
	let sensors: LMSensors = lm_sensors::Initializer::default().initialize().unwrap(); // TODO REFACTORING ME!
	let vbox = vinotebook.append_page(
		&**app_config,
		"lm_sensors",
		sensors
			.version()
			.map(|a| format!("Version: {}", a))
			.as_deref(),
	);
	for chip in sensors.chip_iter(None) {
		vbox.pack_start(
			&ViLabel::new(
				"info_vitextmeter",
				&**app_config,
				&format!("# {} ({})", chip, chip.bus()),
				(),
			)
			.set_margin_top(4)
			.set_margin_start(4)
			.set_margin_bottom(4)
			.set_align(Align::Start)
			.connect_nonblack_background(0.0, 0.0, 0.0, 1.0),
			false,
			false,
			0,
		);

		println!("{} ({})", chip, chip.bus());

		// Print all features of the current chip.
		for feature in chip.feature_iter() {
			if let Some(name) = feature.name().transpose().unwrap() {
				println!("    {}: {}", name, feature);

				#[derive(Debug, Clone, Default)]
				struct Value<'a> {
					input: Option<(f64, Rc<SubFeatureRef<'a>>)>,
					max: Option<(f64, Rc<SubFeatureRef<'a>>)>,
					crit: Option<(f64, Rc<SubFeatureRef<'a>>)>,
					high: Option<(f64, Rc<SubFeatureRef<'a>>)>,
				}

				let mut c_value = Value::default();
				for sub_feature in feature.sub_feature_iter() {
					let sub_feature = Rc::new(sub_feature);
					if let Some(Ok(name)) = sub_feature.name() {
						if name.ends_with("input") {
							if let Ok(value) = sub_feature.value() {
								let v = value.raw_value();
								if v != 0.0 && v < 65261.0 && v > -273.0 {
									c_value.input = (v, sub_feature).into();
								}
							}
						} else if name.ends_with("max") {
							if let Ok(value) = sub_feature.value() {
								let v = value.raw_value();
								if v != 0.0 && v < 65261.0 && v > -273.0 {
									c_value.max = (v, sub_feature).into();
								}
							}
						} else if name.ends_with("high") {
							if let Ok(value) = sub_feature.value() {
								let v = value.raw_value();
								if v != 0.0 && v < 65261.0 && v > -273.0 {
									c_value.high = (v, sub_feature).into();
								}
							}
						} else if name.ends_with("crit") {
							if let Ok(value) = sub_feature.value() {
								let v = value.raw_value();
								if v != 0.0 && v < 65261.0 && v > -273.0 {
									c_value.crit = (v, sub_feature).into();
								}
							}
						}
					}
				}
				if c_value.input.is_some() {
					let vimetr = ViMeter::new_visender(
						app_config.clone(),
						name,
						width,
						200,
						Some(vigraph_surface.clone()),
						1.0,
					);

					vbox.pack_start(&*vimetr, false, false, 0);

					for _ in 0..400 {
						if let Some((input, sub_in)) = &c_value.input {
							if let Some((crit_or_max, sub_crit_or_max)) =
								c_value.crit.as_ref().or(c_value.max.as_ref())
							{
								#[inline]
								const fn map(
									x: f64,
									in_min: f64,
									in_max: f64,
									out_min: f64,
									out_max: f64,
								) -> f64 {
									(x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
								}

								let v = sub_in.raw_value().unwrap();
								let graph = map(v, 0.0, *crit_or_max, 0.0, 1.0);
								vimetr.push_next_and_queue_draw(
									v,
									graph,
									*crit_or_max,
									*crit_or_max,
									0.0,
								);
							} else {
								vimetr.push_next_and_queue_draw(
									sub_in.raw_value().unwrap(),
									(),
									(),
									0.0,
									0.0,
								);
							}
						}
					}
				}
			}
		}
	}
}
