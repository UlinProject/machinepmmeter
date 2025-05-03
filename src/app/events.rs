use async_channel::Receiver;
use async_channel::Sender;
use log::error;

#[inline]
pub fn app_event_channel() -> (AppEventSender, Receiver<AppEvents>) {
	let (tx, rx) = async_channel::unbounded();

	(AppEventSender(tx), rx)
}

#[derive(Debug, Clone, Copy)]
pub enum KeyboardEvents {
	ShiftF8,
	KeypadPlus,
	KeypadMinus,
	DoubleShift,
	Escape,
}

#[derive(Debug, Clone, Copy)]
pub enum AppEvents {
	Keyboard(KeyboardEvents),
	ToggleDockWindowVisibility,
	ShowOrFocusAboutDialog,
	Exit,
	MoveDockWindowToNextPosition,
	KeyboardListenerEnabled(bool),
}

#[derive(Clone)]
pub struct AppEventSender(Sender<AppEvents>);

impl AppEventSender {
	fn __send(&self, ae: AppEvents) {
		if let Err(e) = self.0.send_blocking(ae) {
			error!(
				"#[AppEventSender] I can't send event: {:?}, err: {:?}",
				ae, e
			);
		}
	}

	#[inline]
	pub fn exit(&self) {
		self.__send(AppEvents::Exit);
	}

	#[inline]
	pub fn keyboard_event(&self, e: KeyboardEvents) {
		self.__send(AppEvents::Keyboard(e));
	}

	#[inline]
	pub fn keyboard_listener_enabled(&self, en: bool) {
		self.__send(AppEvents::KeyboardListenerEnabled(en));
	}

	#[inline]
	pub fn toggle_window_visibility(&self) {
		self.__send(AppEvents::ToggleDockWindowVisibility);
	}

	#[inline]
	pub fn move_window_to_next_position(&self) {
		self.__send(AppEvents::MoveDockWindowToNextPosition);
	}

	#[inline]
	pub fn show_or_focus_aboutdialog(&self) {
		self.__send(AppEvents::ShowOrFocusAboutDialog);
	}
}
