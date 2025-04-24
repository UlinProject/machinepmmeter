use gtk::gdk::RGBA;

pub trait ConstOption<T = Self> {
	const IS_SOME: bool;

	#[inline(always)]
	fn is_some(&self) -> bool {
		Self::IS_SOME
	}

	fn value(self) -> T;
}

__always_some_types!(i32, RGBA, &'_ str, String);

impl<T> ConstOption<T> for () {
	const IS_SOME: bool = false;

	#[inline]
	fn value(self) -> T {
		unreachable!()
	}
}

macro_rules! __always_some_types {
	[ $($t:ty),* $(,)? ] => {
		$(
			impl ConstOption for $t {
				const IS_SOME: bool = true;

				#[inline]
				fn value(self) -> Self {
					self
				}
			}
		)*
	};
}
use __always_some_types;

#[macro_export]
macro_rules! fn_const_option {
	[ $name: ident, |$true_v: ident| {$($true_code:tt)*} else || {$($false_code:tt)*} ] => {
		if $name.is_some() {
			let $true_v = $name.value();

			$($true_code)*
		}else {
			$($false_code)*
		}
	};

	[ $name: ident, |$v: ident| $($code:tt)* ] => {
		if $name.is_some() {
			let $v = $name.value();

			$($code)*
		}
	}
}
