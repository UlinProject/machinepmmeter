use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use log::warn;

use crate::widgets::primitives::graph::data::ViGraphData;

pub trait ViGraphStream: Clone + 'static {
	fn with_len(len: usize) -> Self;

	fn read<R>(&self, read: impl FnMut(&ViGraphData) -> R) -> R;
	fn write<R>(&self, write: impl FnMut(&mut ViGraphData) -> R) -> R;
	fn push_next(&self, v: f64);
}

pub type ViGraphRcStream = Rc<RefCell<ViGraphData>>;
impl ViGraphStream for ViGraphRcStream {
	fn with_len(len: usize) -> Self {
		Rc::new(RefCell::new(ViGraphData::with_len(len)))
	}

	fn write<R>(&self, mut write: impl FnMut(&mut ViGraphData) -> R) -> R {
		let mut w = RefCell::borrow_mut(self);

		write(&mut w)
	}

	fn read<R>(&self, mut read: impl FnMut(&ViGraphData) -> R) -> R {
		let rdata = RefCell::borrow(self);

		read(&rdata)
	}

	fn push_next(&self, v: f64) {
		let mut w = RefCell::borrow_mut(self);

		w.push_next(v);
	}
}

pub type ViGraphArcSyncStream = Arc<Mutex<ViGraphData>>;
impl ViGraphStream for ViGraphArcSyncStream {
	fn with_len(len: usize) -> Self {
		Arc::new(Mutex::new(ViGraphData::with_len(len)))
	}

	fn read<R>(&self, mut read: impl FnMut(&ViGraphData) -> R) -> R {
		let rdata = match Mutex::try_lock(self) {
			Ok(a) => a,
			Err(_) => {
				warn!(
					"#[ViGraphStream, read] Fix the timings, rendering is requested at the moment of filling the data."
				);

				self.lock().unwrap_or_else(|e| e.into_inner())
			} // always Err(TryLockError::WouldBlock)
		};

		read(&rdata)
	}

	fn write<R>(&self, mut write: impl FnMut(&mut ViGraphData) -> R) -> R {
		let mut w = self.lock().unwrap_or_else(|e| e.into_inner());

		write(&mut w)
	}

	fn push_next(&self, v: f64) {
		let mut w = self.lock().unwrap_or_else(|e| e.into_inner());

		w.push_next(v);
	}
}
