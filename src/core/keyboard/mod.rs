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

pub struct KeyboardListenerBuilder<const N: usize, KM, EH, SE>
where
	KM: FnOnce(&'_ mut [KeyStateEntry; N]) + Send + Sync + 'static,
	EH: FnMut(&'_ mut [KeyStateEntry; N], Key, ButtonState) + Send + Sync + 'static,
	SE: FnOnce(),
{
	key_mapping: KM,
	handler: EH,
	on_startup: SE,
}

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
	#[allow(dead_code)]
	Pressed,

	#[default]
	Released,
}

impl ButtonState {
	#[allow(dead_code)]
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
	#[allow(dead_code)]
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
	pub const fn get_key(&self) -> Key {
		self.key
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

	#[allow(dead_code)]
	#[inline]
	fn find_entry_mut(&mut self, entry: &KeyStateEntry) -> Option<&mut KeyStateEntry> {
		self.0.iter_mut().find(|elem| **elem == *entry)
	}
}

impl
	KeyboardListenerBuilder<
		0,
		&mut (dyn FnMut(&'_ mut [KeyStateEntry; 0]) + Send + Sync + 'static),
		&mut (dyn FnMut(&'_ mut [KeyStateEntry; 0], Key, ButtonState) + Send + Sync + 'static),
		&mut (dyn FnMut()),
	>
{
	#[allow(clippy::type_complexity)]
	#[inline]
	pub fn with_len<const N: usize>() -> KeyboardListenerBuilder<
		N,
		impl FnOnce(&'_ mut [KeyStateEntry; N]) + Send + Sync + 'static,
		impl FnMut(&'_ mut [KeyStateEntry; N], Key, ButtonState) + Send + Sync + 'static,
		impl FnOnce(),
	> {
		KeyboardListenerBuilder {
			key_mapping: |_w| {},
			handler: |_, _, _| {},
			on_startup: || {},
		}
	}
}

impl<const N: usize, KM, EH, SE> KeyboardListenerBuilder<N, KM, EH, SE>
where
	KM: FnOnce(&'_ mut [KeyStateEntry; N]) + Send + Sync + 'static,
	EH: FnMut(&'_ mut [KeyStateEntry; N], Key, ButtonState) + Send + Sync + 'static,
	SE: FnOnce(),
{
	#[inline]
	pub fn key_mapping<NewKm>(self, key_mapping: NewKm) -> KeyboardListenerBuilder<N, NewKm, EH, SE>
	where
		NewKm: FnOnce(&'_ mut [KeyStateEntry; N]) + Send + Sync + 'static,
	{
		KeyboardListenerBuilder {
			key_mapping,
			handler: self.handler,
			on_startup: self.on_startup,
		}
	}

	#[inline]
	pub fn handler<NewEH>(self, handler: NewEH) -> KeyboardListenerBuilder<N, KM, NewEH, SE>
	where
		NewEH: FnMut(&'_ mut [KeyStateEntry; N], Key, ButtonState) + Send + Sync + 'static,
	{
		KeyboardListenerBuilder {
			key_mapping: self.key_mapping,
			handler,
			on_startup: self.on_startup,
		}
	}

	#[inline]
	pub fn on_startup<NewOS>(self, on_startup: NewOS) -> KeyboardListenerBuilder<N, KM, EH, NewOS>
	where
		NewOS: FnOnce(),
	{
		KeyboardListenerBuilder {
			key_mapping: self.key_mapping,
			handler: self.handler,
			on_startup,
		}
	}

	#[allow(unused_mut)]
	#[allow(unused_variables)]
	pub fn listen(mut self) -> anyhow::Result<()>
	where
		[KeyStateEntry; N]: Default,
	{
		#[cfg(feature = "x11_keyboard")]
		#[cfg_attr(docsrs, doc(cfg(feature = "x11_keyboard")))]
		{
			let mut key_state_table = KeyStateTable::default();
			(self.key_mapping)(&mut key_state_table.0);

			xlib(
				move |key, state| {
					if let Some(key_entry) =
						key_state_table.find_entry_mut(&KeyStateEntry::new(key, state.invert()))
					{
						key_entry.state = state;

						let (key, state) = (key_entry.key, key_entry.state);
						(self.handler)(&mut key_state_table.0, key, state);
					}
				},
				self.on_startup,
			)?;

			return Ok(());
		}

		#[allow(unreachable_code)]
		{
			bail!("Global keypress detection is not supported on this platform.");
		}
	}
}
