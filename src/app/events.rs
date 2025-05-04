use crate::app::keyboard::AppKeyboardEvents;
use async_channel::Receiver;
use async_channel::Sender;
use log::error;
use log::trace;

#[inline]
pub fn app_event_channel() -> (AppEventSender, Receiver<AppEvents>) {
	let (tx, rx) = async_channel::unbounded();

	(AppEventSender(tx), rx)
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
		trace!("#[AppEventSender] exit");
		self.__send(AppEvents::Exit);
	}

	#[inline]
	pub fn keyboard_event(&self, e: AppKeyboardEvents) {
		trace!("#[AppEventSender] keyboard_event: {:?}", e);
		self.__send(AppEvents::Keyboard(e));
	}

	#[inline]
	pub fn keyboard_listener_enabled(&self, en: bool) {
		trace!("#[AppEventSender] keyboard_listener_enabled: {:?}", en);
		self.__send(AppEvents::KeyboardListenerEnabled(en));
	}

	#[inline]
	pub fn toggle_window_visibility(&self) {
		trace!("#[AppEventSender] toggle_window_visibility");
		self.__send(AppEvents::ToggleDockWindowVisibility);
	}

	#[inline]
	pub fn move_window_to_next_position(&self) {
		trace!("#[AppEventSender] move_window_to_next_position");
		self.__send(AppEvents::MoveDockWindowToNextPosition);
	}

	#[inline]
	pub fn move_tab_to_next_position(&self) {
		trace!("#[AppEventSender] move_tab_to_next_position");
		self.__send(AppEvents::MoveTabToNextPosition);
	}

	#[inline]
	pub fn move_tab_to_prev_position(&self) {
		trace!("#[AppEventSender] move_tab_to_prev_position");
		self.__send(AppEvents::MoveTabToPrevPosition);
	}

	#[inline]
	pub fn show_or_focus_aboutdialog(&self) {
		trace!("#[AppEventSender] show_or_focus_aboutdialog");
		self.__send(AppEvents::ShowOrFocusAboutDialog);
	}
}
