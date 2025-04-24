use gtk::gdk::RGBA;

pub trait Maybe<T = Self> {
	const HAS_VALUE: bool;

	#[inline(always)]
	fn has_value(&self) -> bool {
		Self::HAS_VALUE
	}

	fn value(self) -> T;
}

__always_has_value_types!(i32, RGBA, &'_ str, String, usize);

impl<T> Maybe<T> for () {
	const HAS_VALUE: bool = false;

	#[track_caller]
	fn value(self) -> T {
		#[track_caller]
		#[cold]
		fn __cold_panic(v: &str) -> ! {
			panic!("{}", v);
		}

		__cold_panic("Called value() on a Maybe with IS_SOME = false");
	}
}

macro_rules! __always_has_value_types {
	[
		$($t:ty),*

		$(,)?
	] => {
		$(
			impl Maybe for $t {
				const HAS_VALUE: bool = true;

				#[inline(always)]
				fn value(self) -> Self {
					self
				}
			}
		)*
	};
}
use __always_has_value_types;

#[macro_export]
macro_rules! maybe {
	[
		$name: ident, |$true_v: ident| {
			$($true_code:tt)*
		}

		$( else {
			$($false_code:tt)*
		})?
	] => {
		if $name.has_value() {
			let $true_v = $name.value();

			$($true_code)*
		}

		$(else {
			$($false_code)*
		})?
	};

	[
		$name: ident, |$v: ident| $($code:tt)*
	] => {
		if $name.has_value() {
			let $v = $name.value();

			$($code)*
		}
	}
}
