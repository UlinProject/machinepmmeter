use anyhow::bail;
use std::ops::Deref;

use crate::core::keyboard::key::Key;
#[cfg(feature = "x11_keyboard")]
#[cfg_attr(docsrs, doc(cfg(feature = "x11_keyboard")))]
use crate::core::keyboard::x11::xlib;

pub mod key;
#[cfg(feature = "x11_keyboard")]
#[cfg_attr(docsrs, doc(cfg(feature = "x11_keyboard")))]
pub mod x11;

pub struct KeyboardListener;

#[repr(transparent)]
#[derive(Default)]
struct KeyStateTable<const N: usize>([KeyStateEntry; N])
where
	[KeyStateEntry; N]: Default;

#[derive(Debug, PartialEq, Clone)]
pub struct KeyStateEntry {
	key: Key,
	state: ButtonState,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum ButtonState {
	Pressed,

	#[default]
	Released,
}

impl ButtonState {
	#[inline]
	pub const fn invert(self) -> Self {
		match self {
			ButtonState::Pressed => ButtonState::Released,
			ButtonState::Released => ButtonState::Pressed,
		}
	}

	#[inline]
	pub const fn is_pressed(&self) -> bool {
		matches!(self, ButtonState::Pressed)
	}

	#[inline]
	pub const fn is_released(&self) -> bool {
		matches!(self, ButtonState::Released)
	}
}

impl KeyStateEntry {
	#[inline]
	pub const fn new(key: Key, state: ButtonState) -> Self {
		Self { key, state }
	}

	#[inline]
	#[allow(dead_code)]
	pub const fn set_state(&mut self, state: ButtonState) {
		self.state = state;
	}

	#[inline]
	pub const fn set_key(&mut self, key: Key) {
		self.key = key;
	}

	#[inline]
	pub const fn is_pressed(&self) -> bool {
		self.state.is_pressed()
	}

	#[inline]
	#[allow(dead_code)]
	pub const fn is_released(&self) -> bool {
		self.state.is_released()
	}
}

impl Deref for KeyStateEntry {
	type Target = Key;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.key
	}
}

impl Default for KeyStateEntry {
	#[inline]
	fn default() -> Self {
		Self {
			key: Key::Backspace,
			state: Default::default(),
		}
	}
}

impl<const N: usize> KeyStateTable<N>
where
	[KeyStateEntry; N]: Default,
{
	#[allow(dead_code)]
	#[inline]
	pub fn contains_entry(&self, entry: &KeyStateEntry) -> bool {
		self.0.contains(entry)
	}

	#[inline]
	fn find_entry_mut(&mut self, entry: &KeyStateEntry) -> Option<&mut KeyStateEntry> {
		self.0.iter_mut().find(|elem| **elem == *entry)
	}
}

impl KeyboardListener {
	pub fn listen<const N: usize>(
		init_key_table: impl FnOnce(&'_ mut [KeyStateEntry; N]) + Send + Sync + 'static,
		mut event_handler: impl FnMut(&'_ [KeyStateEntry; N], Key, ButtonState) + Send + Sync + 'static,
		success_event: impl FnOnce(),
	) -> anyhow::Result<()>
	where
		[KeyStateEntry; N]: Default,
	{
		#[cfg(feature = "x11_keyboard")]
		#[cfg_attr(docsrs, doc(cfg(feature = "x11_keyboard")))]
		{
			let mut key_state_table = KeyStateTable::default();
			init_key_table(&mut key_state_table.0);

			xlib(
				move |key, state| {
					if let Some(key_entry) =
						key_state_table.find_entry_mut(&KeyStateEntry::new(key, state.invert()))
					{
						key_entry.state = state;

						let (key, state) = (key_entry.key, key_entry.state);
						event_handler(&key_state_table.0, key, state);
					}
				},
				success_event,
			)?;

			return Ok(());
		}

		#[allow(unreachable_code)]
		{
			bail!("Global keypress detection is not supported on this platform.");
		}
	}
}
