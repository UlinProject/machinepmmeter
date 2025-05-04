use crate::app::events::AppEventSender;
use crate::core::keyboard::ButtonState;
use crate::core::keyboard::KeyboardListenerBuilder;
use crate::core::keyboard::key::Key;
use enclose::enc;
use log::error;

#[derive(Debug, Clone, Copy)]
pub enum AppKeyboardEvents {
	ShiftF8,
	KeypadA,
	KeypadD,
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
	KeypadPlus,
	KeypadMinus,
	Escape,
}

pub fn spawn_keyboard_thread(esender: AppEventSender) {
	std::thread::spawn(move || {
		let keyboard_listener = KeyboardListenerBuilder::with_len::<18>()
			.key_mapping(|key_mapping| {
				key_mapping[0].set_key(Key::ShiftRight);
				key_mapping[1].set_key(Key::ShiftLeft);
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
				let astate = (
					(
						state_array[0].is_pressed(), // ShiftRight
						state_array[1].is_pressed(), // ShiftLeft
					),
					state_array[2].is_pressed(), // F8
					state_array[3].is_pressed(), // KeypadPlus
					state_array[4].is_pressed(), // KeypadMinus
					state_array[5].is_pressed(), // Escape
					state_array[6].is_pressed(), // KeyA
					state_array[7].is_pressed(), // KeyD
					state_array[8].is_pressed(), // Key1
					state_array[9].is_pressed(), // Key2
					state_array[10].is_pressed(), // Key3
					state_array[11].is_pressed(), // Key4
					state_array[12].is_pressed(), // Key5
					state_array[13].is_pressed(), // Key6
					state_array[14].is_pressed(), // Key7
					state_array[15].is_pressed(), // Key8
					state_array[16].is_pressed(), // Key9
					state_array[17].is_pressed(), // KeyP
				);
				let sendevent = |e| {
					esender.keyboard_event(e);
				};
				match astate {
					((true, true), ..) => {
						// L+R SHIFT
					}
					((true, false) | (false, true), true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false) => {
						// L/R SHIFT + F8
						sendevent(AppKeyboardEvents::ShiftF8);
					}
					((true, false) | (false, true), false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false) => {
						// L/R SHIFT + KeypadPlus
						sendevent(AppKeyboardEvents::KeypadPlus);
					}
					((true, false) | (false, true), false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false) => {
						// L/R SHIFT + KeypadMinus
						sendevent(AppKeyboardEvents::KeypadMinus);
					}
					((true, false) | (false, true), false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false) => {
						// L/R SHIFT + Escape
						sendevent(AppKeyboardEvents::Escape);
					}
					((true, false) | (false, true), false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false) => {
						// L/R SHIFT + A
						sendevent(AppKeyboardEvents::KeypadA);
					}
					((true, false) | (false, true), false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false) => {
						// L/R SHIFT + D
						sendevent(AppKeyboardEvents::KeypadD);
					}
					((true, false) | (false, true), false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false) => {
						// L/R SHIFT + 1
						sendevent(AppKeyboardEvents::Num1);
					}
					((true, false) | (false, true), false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false) => {
						// L/R SHIFT + 2
						sendevent(AppKeyboardEvents::Num2);
					}
					((true, false) | (false, true), false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false) => {
						// L/R SHIFT + 3
						sendevent(AppKeyboardEvents::Num3);
					}
					((true, false) | (false, true), false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false) => {
						// L/R SHIFT + 4
						sendevent(AppKeyboardEvents::Num4);
					}
					((true, false) | (false, true), false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false) => {
						// L/R SHIFT + 5
						sendevent(AppKeyboardEvents::Num5);
					}
					((true, false) | (false, true), false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false) => {
						// L/R SHIFT + 6
						sendevent(AppKeyboardEvents::Num6);
					}
					((true, false) | (false, true), false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false) => {
						// L/R SHIFT + 7
						sendevent(AppKeyboardEvents::Num7);
					}
					((true, false) | (false, true), false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false) => {
						// L/R SHIFT + 8
						sendevent(AppKeyboardEvents::Num8);
					}
					((true, false) | (false, true), false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false) => {
						// L/R SHIFT + 9
						sendevent(AppKeyboardEvents::Num9);
					},
					((true, false) | (false, true), false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true) => {
						// L/R SHIFT + 9
						sendevent(AppKeyboardEvents::KeyP);
					}
					_ => {}
				}
			})).on_startup(|| {
				esender.keyboard_listener_enabled(true);
			}).listen();

		if let Err(e) = keyboard_listener {
			error!(
				"#[global keyboard] Error initializing global keyboard listener, keyboard shortcuts not available. {}",
				e
			);
			esender.keyboard_listener_enabled(false);
		}
	});
}
