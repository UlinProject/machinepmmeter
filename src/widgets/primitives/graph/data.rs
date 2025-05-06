use std::collections::VecDeque;
use log::warn;

#[repr(transparent)]
pub struct ViGraphData(VecDeque<f64>);

impl ViGraphData {
	pub fn with_len(len: usize) -> Self {
		Self(VecDeque::from(vec![0.0; len]))
	}

	#[inline]
	pub fn back(&self) -> Option<f64> {
		self.0.back().copied()
	}

	#[inline]
	pub fn front(&self) -> Option<f64> {
		self.0.front().copied()
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[inline]
	pub fn iter(&self) -> impl Iterator<Item = &f64> {
		self.0.iter()
	}

	#[inline]
	pub fn push_next(&mut self, mut v: f64) {
		if !(0.0..=1.0).contains(&v) {
			warn!("#[ViGraphData, push_next] Very strange {}f for a graph.", v);

			v = v.clamp(0.0, 1.0);
		}

		self.0.pop_front();
		self.0.push_back(v);
	}
}
