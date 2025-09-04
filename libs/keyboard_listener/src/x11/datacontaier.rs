use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr;
use std::sync::Mutex;

/// Vulnerable numbers to detect overwriting
#[cfg(target_pointer_width = "64")]
const ALTERNATING_MASK: usize = 0xAAAAAAAAAAAAAAAA;

/// Vulnerable numbers to detect overwriting
#[cfg(target_pointer_width = "32")]
const ALTERNATING_MASK: usize = 0xAAAAAAAA;


/// Since extern can be called by anyone at any time, and I don't want to make `callback`
/// static... you could do pointer substitution directly from raw_record_callback,
/// but you don't want to trust that.
/// so I'll introduce three identifiers, two of which will be immutable (these can be used to
/// detect that the pointer has been freed or even the data has been overwritten by someone else, i.e. it wasn't us),
/// the second will be a Mutex with Option some, which is more reliable and will
/// indicate that the thread is still processing and can run, or the thread
/// has long since finished its work)
#[derive(Debug)]
pub struct ExterDataContainer<T> {
	lpadding: usize,
	data: Mutex<Option<T>>,
	rpadding: usize,
}

impl<T> ExterDataContainer<T> {
	pub fn container(data: T) -> SafeDropExternDataContainer<T> {
		let data = Box::new(Self {
			lpadding: ALTERNATING_MASK,
			data: Mutex::new(Some(data)),
			rpadding: ALTERNATING_MASK,
		});

		SafeDropExternDataContainer(data)
	}

	pub fn check_and_lock<R>(&self, mut next: impl FnMut(&mut T) -> R) -> Option<R> {
		if self.lpadding == ALTERNATING_MASK && self.rpadding == ALTERNATING_MASK {
			if let Ok(mut guard) = self.data.lock() {
				if self.lpadding == ALTERNATING_MASK && self.rpadding == ALTERNATING_MASK {
					if let Some(ldata) = &mut *guard {
						return Some(next(ldata));
					}
				}
			}
		}

		None
	}
}

/// The root owner of `ExterDataContainer` must stop the container when `Drop` is
/// called to prevent it from being used when `extern` is called.
#[repr(transparent)]
pub struct SafeDropExternDataContainer<T>(Box<ExterDataContainer<T>>);

impl<T> SafeDropExternDataContainer<T> {
	#[inline]
	pub fn as_ptr(&self) -> *const ExterDataContainer<T> {
		Box::deref(&self.0) as *const _
	}
}

impl<T> Deref for SafeDropExternDataContainer<T> {
	type Target = ExterDataContainer<T>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		Box::deref(&self.0)
	}
}

impl<T> DerefMut for SafeDropExternDataContainer<T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		Box::deref_mut(&mut self.0)
	}
}

impl<T> Drop for SafeDropExternDataContainer<T> {
	fn drop(&mut self) {
		// Force the compiler to create this entry even if it thinks it's unnecessary, 
		// although on Box it may not be necessary.
		unsafe {
			ptr::write_volatile(&mut self.lpadding, 0);
			ptr::write_volatile(&mut self.rpadding, 0);
		}
		
		*(self.0.data.lock().unwrap_or_else(|e| e.into_inner())) = None;
	}
}
