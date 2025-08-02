
macro_rules! toml_decomment {
	[ // comment
		@in [
			# $_: ident = $_2:tt
			$($all:tt)*
		]
		@result [ $($result:tt)* ]
		@end [ $($end:tt)* ]
	] => {
		$crate::toml_decomment! {
			@in [ $($all)* ]
			@result [ $($result)* ]
			@end [ $($end)* ]
		}
	};
	
	[ // header
		@in [
			[$($data:tt)*]
			$($all:tt)*
		]
		@result [ $($result:tt)* ]
		@end [ $($end:tt)* ]
	] => {
		$crate::toml_decomment! {
			@in [ $($all)* ]
			@result [
				$($result)*
				[$($data)*]
			]
			@end [ $($end)* ]
		}
	};
	[ // value
		@in [
			$name: ident = $expr:tt
			$($all:tt)*
		]
		@result [ $($result:tt)* ]
		@end [ $($end:tt)* ]
	] => {
		$crate::toml_decomment! {
			@in [ $($all)* ]
			@result [
				$($result)*
				$name = $expr
			]
			@end [ $($end)* ]
		}
	};
	
	
	[ // end
		@in []
		@result [ $($result:tt)* ]
		@end [ $($path:tt)* ]
	] => {
		$($path)* ! {
			$($result)*
		}
	};
}

pub (crate) use toml_decomment;