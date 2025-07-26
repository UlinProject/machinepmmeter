use crate::app::keyboard::AppKeyboardEvents;
use async_channel::Receiver;
use async_channel::Sender;
use log::error;
use log::trace;

#[inline]
pub fn app_events_channel() -> (AppEventsSender, Receiver<AppEvents>) {
	let (tx, rx) = async_channel::unbounded();

	(AppEventsSender(tx), rx)
}

#[derive(Debug, Clone, Copy)]
pub enum AppEvents {
	Keyboard(AppKeyboardEvents),
	ToggleDockWindowVisibility,
	ShowOrFocusAboutDialog,
	Exit,
	MoveDockWindowToNextPosition,
	MoveTabToPrevPosition,
	MoveTabToNextPosition,
	KeyboardListenerEnabled(bool),
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AppEventsSender(Sender<AppEvents>);

impl AppEventsSender {
	fn __send(&self, ae: AppEvents) {
		if let Err(e) = self.0.send_blocking(ae) {
			error!(
				"#[AppEventsSender] I can't send event: {:?}, err: {:?}",
				ae, e
			);
		}
	}

	#[inline]
	pub fn exit(&self) {
		trace!("#[AppEventsSender] exit");
		self.__send(AppEvents::Exit);
	}

	#[inline]
	pub fn keyboard_event(&self, e: AppKeyboardEvents) {
		trace!("#[AppEventsSender] keyboard_event: {:?}", e);
		self.__send(AppEvents::Keyboard(e));
	}

	#[inline]
	pub fn keyboard_listener_enabled(&self, en: bool) {
		trace!("#[AppEventsSender] keyboard_listener_enabled: {:?}", en);
		self.__send(AppEvents::KeyboardListenerEnabled(en));
	}

	#[inline]
	pub fn toggle_window_visibility(&self) {
		trace!("#[AppEventsSender] toggle_window_visibility");
		self.__send(AppEvents::ToggleDockWindowVisibility);
	}

	#[inline]
	pub fn move_window_to_next_position(&self) {
		trace!("#[AppEventsSender] move_window_to_next_position");
		self.__send(AppEvents::MoveDockWindowToNextPosition);
	}

	#[inline]
	pub fn move_tab_to_next_position(&self) {
		trace!("#[AppEventsSender] move_tab_to_next_position");
		self.__send(AppEvents::MoveTabToNextPosition);
	}

	#[inline]
	pub fn move_tab_to_prev_position(&self) {
		trace!("#[AppEventsSender] move_tab_to_prev_position");
		self.__send(AppEvents::MoveTabToPrevPosition);
	}

	#[inline]
	pub fn show_or_focus_aboutdialog(&self) {
		trace!("#[AppEventsSender] show_or_focus_aboutdialog");
		self.__send(AppEvents::ShowOrFocusAboutDialog);
	}
}
