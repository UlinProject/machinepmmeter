
macro_rules! no_comments_toml {
	[ // comment
		@in [
			#$_: ident = $_2:tt
			$($all:tt)*
		]
		@result [ $($result:tt)* ]
		@end [ $($end:tt)* ]
	] => {
		$crate::no_comments_toml! {
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
		$crate::no_comments_toml! {
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
		$crate::no_comments_toml! {
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

pub (crate) use no_comments_toml;