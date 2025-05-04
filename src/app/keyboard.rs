use crate::app::events::AppEventSender;
use crate::core::keyboard::KeyboardListenerBuilder;
use crate::core::keyboard::key::Key;
use enclose::enc;
use log::error;

#[derive(Debug, Clone, Copy)]
pub enum AppKeyboardEvents {
	ShiftF8,
	KeyA,
	KeyD,
	KeyP,
	Num1,
	Num2,
	Num3,
	Num4,
	Num5,
	Num6,
	Num7,
	Num8,
	Num9,
	KeyPlus,
	KeyMinus,
	Escape,
}

pub fn spawn_keyboard_thread(esender: AppEventSender) {
	std::thread::spawn(move || {
		let keyboard_listener = KeyboardListenerBuilder::with_len::<18>()
			.key_mapping(|key_mapping| {
				key_mapping[0].set_key(Key::ShiftLeft);
				key_mapping[1].set_key(Key::ShiftRight);
				key_mapping[2].set_key(Key::F8);
				key_mapping[3].set_key(Key::KpPlus);
				key_mapping[4].set_key(Key::KpMinus);
				key_mapping[5].set_key(Key::Escape);
				key_mapping[6].set_key(Key::KeyA);
				key_mapping[7].set_key(Key::KeyD);
				key_mapping[8].set_key(Key::Num1);
				key_mapping[9].set_key(Key::Num2);
				key_mapping[10].set_key(Key::Num3);
				key_mapping[11].set_key(Key::Num4);
				key_mapping[12].set_key(Key::Num5);
				key_mapping[13].set_key(Key::Num6);
				key_mapping[14].set_key(Key::Num7);
				key_mapping[15].set_key(Key::Num8);
				key_mapping[16].set_key(Key::Num9);
				key_mapping[17].set_key(Key::KeyP);
			})
			.handler(enc!((esender) move |state_array, _key, _state| {
				let mut sa_iter = state_array.iter();
				match (
					sa_iter.next(), // ShiftLeft
					sa_iter.next(), // ShiftRight
				) {
					(Some(left), Some(right)) => {
						let left = left.is_pressed();
						let right = right.is_pressed();

						if (left && !right) || (!left && right) {
							let mut pressed_key = None;
							for astate in sa_iter {
								if astate.is_pressed() {
									if pressed_key.is_some() {
										return;
									}
									pressed_key = Some(astate.get_key());
								}
							}
							esender.keyboard_event(match pressed_key {
								Some(Key::F8) => AppKeyboardEvents::ShiftF8,
								Some(Key::KpPlus) => AppKeyboardEvents::KeyPlus,
								Some(Key::KpMinus) => AppKeyboardEvents::KeyMinus,
								Some(Key::Escape) => AppKeyboardEvents::Escape,
								Some(Key::KeyA) => AppKeyboardEvents::KeyA,
								Some(Key::KeyD) => AppKeyboardEvents::KeyD,
								Some(Key::Num1) => AppKeyboardEvents::Num1,
								Some(Key::Num2) => AppKeyboardEvents::Num2,
								Some(Key::Num3) => AppKeyboardEvents::Num3,
								Some(Key::Num4) => AppKeyboardEvents::Num4,
								Some(Key::Num5) => AppKeyboardEvents::Num5,
								Some(Key::Num6) => AppKeyboardEvents::Num6,
								Some(Key::Num7) => AppKeyboardEvents::Num7,
								Some(Key::Num8) => AppKeyboardEvents::Num8,
								Some(Key::Num9) => AppKeyboardEvents::Num9,
								Some(Key::KeyP) => AppKeyboardEvents::KeyP,
								_ => return,
							});
						}
					},
					_ => {},
				};
			}))
			.on_startup(|| {
				esender.keyboard_listener_enabled(true);
			})
			.listen();

		if let Err(e) = keyboard_listener {
			error!(
				"#[global keyboard] Error initializing global keyboard listener, keyboard shortcuts not available. {}",
				e
			);
			esender.keyboard_listener_enabled(false);
		}
	});
}
