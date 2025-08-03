macro_rules! parse_and_impl_theme_toml {
	[ // START
		[theme]
		$( $name: tt = $expr:tt )*
	] => {
		$crate::parse_and_impl_theme_toml! {
			@toml_data [ // default data
				name = ["unknown"]
				main_css = ["main.css"]
				version = ["0.0.0"]
				description = [""]
			]

			$([
				[$name][$expr]
			])*
		}
	};
	[
		@toml_data [
			name = [ $($name:tt)* ]
			main_css = [ $($main_css:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]

		[[name][$expr:expr]]
		$($all:tt)*
	] => {
		$crate::parse_and_impl_theme_toml! {
			@toml_data [ // default data
				name = [$expr]
				main_css = [$($main_css)*]
				version = [$($version)*]
				description = [$($description)*]
			]

			$($all)*
		}
	};
	[
		@toml_data [
			name = [ $($name:tt)* ]
			main_css = [ $($main_css:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]

		[[main_css][$expr:expr]]
		$($all:tt)*
	] => {
		$crate::parse_and_impl_theme_toml! {
			@toml_data [
				name = [$($name)*]
				main_css = [$expr]
				version = [$($version)*]
				description = [$($description)*]
			]

			$($all)*
		}
	};
	[
		@toml_data [
			name = [ $($name:tt)* ]
			main_css = [ $($main_css:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]

		[[version][$expr:expr]]
		$($all:tt)*
	] => {
		$crate::parse_and_impl_theme_toml! {
			@toml_data [
				name = [$($name)*]
				main_css = [$($main_css)*]
				version = [$expr]
				description = [$($description)*]
			]

			$($all)*
		}
	};
	[
		@toml_data [
			name = [ $($name:tt)* ]
			main_css = [ $($main_css:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]

		[[description][$expr:expr]]
		$($all:tt)*
	] => {
		$crate::parse_and_impl_theme_toml! {
			@toml_data [
				name = [$($name)*]
				main_css = [$($main_css)*]
				version = [$($version)*]
				description = [$expr]
			]

			$($all)*
		}
	};

	[ // END
		@toml_data [
			name = [ $name: expr ]
			main_css = [ $main_css: expr ]
			version = [ $version: expr ]
			description = [ $description: expr ]
		]
	] => {
		pub const A_THEME_NAME: &str = $name;
		pub const A_THEME_VERSION: &str = $version;
		pub const A_THEME_DESCRIPTION: &str = $description;

		$crate::inject! {
			pub const A_THEME_MAIN_CSS: &[u8] = #arr("themes/" $name / $main_css);
		}
	};
}

pub(crate) use parse_and_impl_theme_toml;
