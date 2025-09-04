// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Denis Kotlyarov (Денис Котляров) <denis2005991@gmail.com>

#[macro_export]
macro_rules! maybetype {
	[
		$($name:ident: ( $($t:ty),* $(,)? ) ;)*
	] => {
		$(
			pub trait $name <T = Self> {
				const HAS_VALUE: bool;

				#[inline(always)]
				fn has_value(&self) -> bool {
					Self::HAS_VALUE
				}

				fn value(self) -> T;
			}

			impl<T> $name <T> for () {
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

			$(
				impl $name for $t {
					const HAS_VALUE: bool = true;

					#[inline(always)]
					fn value(self) -> Self {
						self
					}
				}
			)*
		)*
	};
}

#[macro_export]
macro_rules! maybe {
	[
		($name: ident) {
			$($true_code:tt)*
		}

		$( else {
			$($false_code:tt)*
		})?
	] => {
		if $name.has_value() {
			let $name = $name.value();

			$($true_code)*
		}

		$(else {
			$($false_code)*
		})?
	};

	[
		($name: ident) $($code:tt)+
	] => {
		if $name.has_value() {
			let $name = $name.value();

			$($code)*
		}
	};

	[
		($name: ident)
	] => {
		if $name.has_value() {
			$name.value()
		} else {
			Default::default()
		}
	};
}
