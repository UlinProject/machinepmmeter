#[macro_export]
macro_rules! __gen_transparent_gtk_type {
	(
		#[sys($systy: ty)]
		$forty: tt (
			new |$sself_name_for_new: ident : $sselty: ty| {
				$($code_new: tt)* // new
			},
			ref |$sself_name_for_ref: ident| {
				$($code_ref: tt)* // get
			},
			into |$sself_name_for_into: ident| {
				$($code_into: tt)* // into
			} $(,)?
		) $(,)?
	) => {
		use std::borrow::Borrow;
		use std::hash::Hash;
		use gtk::{
			Widget,
			glib::{
				IsA, StaticType, ToValue,
				object::ObjectType,
				translate::{IntoGlibPtr, ToGlibPtr, UnsafeFrom},
				value::{FromValue, ToValueOptional, ValueType},
			},
		};
		use std::cmp::Ordering;
		use gtk::glib::object::ObjectRef;
		use std::hash::Hasher;

		impl AsRef<Widget> for $forty {
			#[inline]
			fn as_ref(&self) -> &Widget {
				let $sself_name_for_ref = self;

				AsRef::<Widget>::as_ref($($code_ref)*)
			}
		}

		impl Borrow<Widget> for $forty {
			#[inline]
			fn borrow(&self) -> &Widget {
				let $sself_name_for_ref = self;

				Borrow::<Widget>::borrow($($code_ref)*)
			}
		}

		impl IntoGlibPtr<*mut $systy> for $forty {
			#[inline]
			unsafe fn into_glib_ptr(self) -> *mut $systy {
				let $sself_name_for_into = self;
				let value = {
					$($code_into)*
				};

				unsafe { <$sselty as IntoGlibPtr<*mut $systy>>::into_glib_ptr(value) }
			}
		}

		impl<'a> ToGlibPtr<'a, *mut $systy> for $forty {
			type Storage = <$sselty as ToGlibPtr<'a, *mut $systy>>::Storage;

			#[inline]
			fn to_glib_none(&'a self) -> gtk::glib::translate::Stash<'a, *mut $systy, Self> {
				let $sself_name_for_ref = self;
				let stash = <$sselty as ToGlibPtr<'a, *mut $systy>>::to_glib_none($($code_ref)*);

				gtk::glib::translate::Stash(stash.0, stash.1)
			}
		}

		impl ToValueOptional for $forty {
			#[inline]
			fn to_value_optional(s: Option<&Self>) -> gtk::glib::Value {
				<$sselty as ToValueOptional>::to_value_optional(s.map(|v| {
					let $sself_name_for_ref = v;

					$($code_ref)*
				}))
			}
		}

		unsafe impl<'a> FromValue<'a> for $forty {
			type Checker = <$sselty as FromValue<'a>>::Checker;

			#[inline]
			unsafe fn from_value(value: &'a gtk::glib::Value) -> Self {
				let $sself_name_for_new = unsafe { <$sselty as FromValue<'a>>::from_value(value) };

				{$($code_new)*}
			}
		}

		impl ValueType for $forty {
			type Type = <$sselty as ValueType>::Type;
		}

		impl ToValue for $forty {
			#[inline]
			fn to_value(&self) -> gtk::glib::Value {
				let $sself_name_for_ref = self;

				<$sselty as ToValue>::to_value($($code_ref)*)
			}

			#[inline]
			fn value_type(&self) -> gtk::glib::Type {
				let $sself_name_for_ref = self;

				<$sselty as ToValue>::value_type($($code_ref)*)
			}
		}

		impl Hash for $forty {
			#[inline]
			fn hash<H: Hasher>(&self, state: &mut H) {
				let $sself_name_for_ref = self;

				<$sselty as Hash>::hash($($code_ref)*, state)
			}
		}

		impl PartialEq for $forty {
			#[inline]
			fn eq(&self, other: &Self) -> bool {
				let sself = {
					let $sself_name_for_ref = self;

					$($code_ref)*
				};
				let other = {
					let $sself_name_for_ref = other;

					$($code_ref)*
				};

				<$sselty as PartialEq>::eq(sself, other)
			}
		}

		#[allow(clippy::non_canonical_partial_ord_impl)]
		impl PartialOrd for $forty {
			#[inline]
			fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
				let sself = {
					let $sself_name_for_ref = self;

					$($code_ref)*
				};
				let other = {
					let $sself_name_for_ref = other;

					$($code_ref)*
				};

				<$sselty as PartialOrd>::partial_cmp(sself, other)
			}

			#[inline]
			fn lt(&self, other: &Self) -> bool {
				let sself = {
					let $sself_name_for_ref = self;

					$($code_ref)*
				};
				let other = {
					let $sself_name_for_ref = other;

					$($code_ref)*
				};

				<$sselty as PartialOrd>::lt(sself, other)
			}

			#[inline]
			fn le(&self, other: &Self) -> bool {
				let sself = {
					let $sself_name_for_ref = self;

					$($code_ref)*
				};
				let other = {
					let $sself_name_for_ref = other;

					$($code_ref)*
				};

				<$sselty as PartialOrd>::le(sself, other)
			}

			#[inline]
			fn gt(&self, other: &Self) -> bool {
				let sself = {
					let $sself_name_for_ref = self;

					$($code_ref)*
				};
				let other = {
					let $sself_name_for_ref = other;

					$($code_ref)*
				};

				<$sselty as PartialOrd>::gt(sself, other)
			}

			#[inline]
			fn ge(&self, other: &Self) -> bool {
				let sself = {
					let $sself_name_for_ref = self;

					$($code_ref)*
				};
				let other = {
					let $sself_name_for_ref = other;

					$($code_ref)*
				};

				<$sselty as PartialOrd>::ge(sself, other)
			}
		}

		impl Eq for $forty {}

		impl Clone for $forty {
			#[inline]
			fn clone(&self) -> Self {
				let $sself_name_for_new = {
					let $sself_name_for_ref = self;

					<$sselty as Clone>::clone($($code_ref)*)
				};

				$($code_new)*
			}
		}

		impl Ord for $forty {
			#[inline]
			fn cmp(&self, other: &Self) -> Ordering {
				let sself = {
					let $sself_name_for_ref = self;

					$($code_ref)*
				};
				let other = {
					let $sself_name_for_ref = other;

					$($code_ref)*
				};

				<$sselty as Ord>::cmp(sself, other)
			}
		}

		impl StaticType for $forty {
			#[inline]
			fn static_type() -> gtk::glib::Type {
				<$sselty as StaticType>::static_type()
			}
		}

		impl UnsafeFrom<ObjectRef> for $forty {
			#[inline]
			unsafe fn unsafe_from(t: ObjectRef) -> Self {
				let $sself_name_for_new = unsafe { <$sselty as UnsafeFrom<ObjectRef>>::unsafe_from(t) };

				$($code_new)*
			}
		}

		impl From<$forty> for ObjectRef {
			#[inline]
			fn from(val: $forty) -> Self {
				let $sself_name_for_into = val;

				<$sselty as Into<ObjectRef>>::into($($code_into)* as $sselty)
			}
		}

		unsafe impl ObjectType for $forty {
			type GlibType = <$sselty as ObjectType>::GlibType;
			type GlibClassType = <$sselty as ObjectType>::GlibClassType;

			#[inline]
			fn as_object_ref(&self) -> &ObjectRef {
				let $sself_name_for_ref = self;

				<$sselty as ObjectType>::as_object_ref($($code_ref)*)
			}

			#[inline]
			fn as_ptr(&self) -> *mut Self::GlibType {
				let $sself_name_for_ref = self;

				<$sselty as ObjectType>::as_ptr($($code_ref)*)
			}

			unsafe fn from_glib_ptr_borrow<'a>(ptr: *const *const Self::GlibType) -> &'a Self {
				// TODO
				let label: &$sselty = unsafe { <$sselty as ObjectType>::from_glib_ptr_borrow(ptr) };

				// strange moment
				unsafe { &*((label as &$sselty) as *const $sselty as *const $forty) as &$forty }
			}
		}

		impl From<$forty> for Widget {
			#[inline]
			fn from(val: $forty) -> Self {
				let $sself_name_for_into = val;

				<$sselty as Into<Widget>>::into($($code_into)* as $sselty)
			}
		}

		unsafe impl IsA<Widget> for $forty {}
	}
}
