macro_rules! parse_and_impl_project_toml {
	// CONFIG
	[ // START
		$( @active[$($_:tt)*] )?
		$( @config [ $($config_data:tt)* ] )?
		$( @app [ $($app_data:tt)* ] )?

		[ config ]
		$( $name: ident = $expr:tt )*
		$( [$($n_tt:tt)*] $($all:tt)*)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[config]
			@config [ // default data
				qualifier = [ $crate::default_project_toml::CONFIG_QUALIFIER ]
				organization = [ $crate::default_project_toml::CONFIG_ORGANIZATION ]
				file_name = [ $crate::default_project_toml::CONFIG_FILE_NAME ]
			]
			$( @app [ $($app_data)* ] )?

			$(#[ [$name][$expr] ];)*
			$( [$($n_tt)*] $($all)* )*
		}
	};
	[
		@active[config]
		@config [
			qualifier = [ $($qualifier:tt)* ]
			organization = [ $($organization:tt)* ]
			file_name = [ $($file_name:tt)* ]
		]
		$( @app_data [ $($app_data:tt)* ] )?

		#[[ qualifier ][ $expr:expr ]];
		$($all:tt)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[config]
			@config [ // default data
				qualifier = [ $expr ]
				organization = [ $($organization)* ]
				file_name = [ $($file_name)* ]
			]
			$( @app [ $($app_data)* ] )?

			$($all)*
		}
	};
	[
		@active[config]
		@config [
			qualifier = [ $($qualifier:tt)* ]
			organization = [ $($organization:tt)* ]
			file_name = [ $($file_name:tt)* ]
		]
		$( @app_data [ $($app_data:tt)* ] )?

		#[[organization][$expr:expr]];
		$($all:tt)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[config]
			@config [
				qualifier = [ $($qualifier)* ]
				organization = [ $expr ]
				file_name = [ $($file_name)* ]
			]
			$( @app [ $($app_data)* ] )?

			$($all)*
		}
	};
	[
		@active[config]
		@config [
			qualifier = [ $($qualifier:tt)* ]
			organization = [ $($organization:tt)* ]
			file_name = [ $($file_name:tt)* ]
		]
		$( @app_data [ $($app_data:tt)* ] )?

		#[[file_name][$expr:expr]];
		$($all:tt)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[config]
			@config [
				qualifier = [ $($qualifier)* ]
				organization = [ $($organization)* ]
				file_name = [ $expr ]
			]
			$( @app [ $($app_data)* ] )?

			$($all)*
		}
	};
	// END CONFIG

	// APP
	[ // START
		$( @active[$($_:tt)*] )?
		$( @config [ $($config_data:tt)* ] )?
		$( @app [ $($app_data:tt)* ] )?

		[ app ]
		$( $name: ident = $expr:tt )*
		$( [$($n_tt:tt)*] $($all:tt)*)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[app]
			$( @config [ $($config_data)* ] )?
			@app [ // default data
				icon = [ $crate::default_project_toml::APP_ICON ]
				website = [ $crate::default_project_toml::APP_WEBSITE ]
				authors = [ $crate::default_project_toml::APP_AUTHORS ]
				copyright = [ $crate::default_project_toml::APP_COPYRIGHT ]
				name = [ $crate::default_project_toml::APP_NAME ]
				version = [ $crate::default_project_toml::APP_VERSION ]
				description = [ $crate::default_project_toml::APP_DESCRIPTION ]
			]

			$(#[ [$name][$expr] ];)*
			$( [$($n_tt)*] $($all)* )*
		}
	};

	[
		@active[app]
		$( @config [ $($config_data:tt)* ] )?
		@app [
			icon = [ $($icon:tt)* ]
			website = [ $($website:tt)* ]
			authors = [ $($authors:tt)* ]
			copyright = [ $($copyright:tt)* ]
			name = [ $($name:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]

		#[[ icon ][ $expr:expr ]];
		$($all:tt)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[app]
			$( @config [ $($config_data)* ] )?
			@app [
				icon = [ $expr ]
				website = [ $($website)* ]
				authors = [ $($authors)* ]
				copyright = [ $($copyright)* ]
				name = [ $($name)* ]
				version = [ $($version)* ]
				description = [ $($description)* ]
			]

			$($all)*
		}
	};
	[
		@active[app]
		$( @config [ $($config_data:tt)* ] )?
		@app [
			icon = [ $($icon:tt)* ]
			website = [ $($website:tt)* ]
			authors = [ $($authors:tt)* ]
			copyright = [ $($copyright:tt)* ]
			name = [ $($name:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]

		#[[ website ][ $expr:expr ]];
		$($all:tt)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[app]
			$( @config [ $($config_data)* ] )?
			@app [
				icon = [ $($icon)* ]
				website = [ $expr ]
				authors = [ $($authors)* ]
				copyright = [ $($copyright)* ]
				name = [ $($name)* ]
				version = [ $($version)* ]
				description = [ $($description)* ]
			]

			$($all)*
		}
	};
	[
		@active[app]
		$( @config [ $($config_data:tt)* ] )?
		@app [
			icon = [ $($icon:tt)* ]
			website = [ $($website:tt)* ]
			authors = [ $($authors:tt)* ]
			copyright = [ $($copyright:tt)* ]
			name = [ $($name:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]

		#[[ authors ][ $expr:expr ]];
		$($all:tt)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[app]
			$( @config [ $($config_data)* ] )?
			@app [
				icon = [ $($icon)* ]
				website = [ $($website)* ]
				authors = [ $expr ]
				copyright = [ $($copyright)* ]
				name = [ $($name)* ]
				version = [ $($version)* ]
				description = [ $($description)* ]
			]

			$($all)*
		}
	};
	[
		@active[app]
		$( @config [ $($config_data:tt)* ] )?
		@app [
			icon = [ $($icon:tt)* ]
			website = [ $($website:tt)* ]
			authors = [ $($authors:tt)* ]
			copyright = [ $($copyright:tt)* ]
			name = [ $($name:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]

		#[[ copyright ][ $expr:expr ]];
		$($all:tt)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[app]
			$( @config [ $($config_data)* ] )?
			@app [
				icon = [ $($icon)* ]
				website = [ $($website)* ]
				authors = [ $($authors)* ]
				copyright = [ $expr ]
				name = [ $($name)* ]
				version = [ $($version)* ]
				description = [ $($description)* ]
			]

			$($all)*
		}
	};
	[
		@active[app]
		$( @config [ $($config_data:tt)* ] )?
		@app [
			icon = [ $($icon:tt)* ]
			website = [ $($website:tt)* ]
			authors = [ $($authors:tt)* ]
			copyright = [ $($copyright:tt)* ]
			name = [ $($name:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]

		#[[ name ][ $expr:expr ]];
		$($all:tt)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[app]
			$( @config [ $($config_data)* ] )?
			@app [
				icon = [ $($icon)* ]
				website = [ $($website)* ]
				authors = [ $($authors)* ]
				copyright = [ $($copyright)* ]
				name = [ $expr ]
				version = [ $($version)* ]
				description = [ $($description)* ]
			]

			$($all)*
		}
	};
	[
		@active[app]
		$( @config [ $($config_data:tt)* ] )?
		@app [
			icon = [ $($icon:tt)* ]
			website = [ $($website:tt)* ]
			authors = [ $($authors:tt)* ]
			copyright = [ $($copyright:tt)* ]
			name = [ $($name:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]

		#[[ version ][ $expr:expr ]];
		$($all:tt)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[app]
			$( @config [ $($config_data)* ] )?
			@app [
				icon = [ $($icon)* ]
				website = [ $($website)* ]
				authors = [ $($authors)* ]
				copyright = [ $($copyright)* ]
				name = [ $($name)* ]
				version = [ $expr ]
				description = [ $($description)* ]
			]

			$($all)*
		}
	};
	[
		@active[app]
		$( @config [ $($config_data:tt)* ] )?
		@app [
			icon = [ $($icon:tt)* ]
			website = [ $($website:tt)* ]
			authors = [ $($authors:tt)* ]
			copyright = [ $($copyright:tt)* ]
			name = [ $($name:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]

		#[[ description ][ $expr:expr ]];
		$($all:tt)*
	] => {
		$crate::parse_and_impl_project_toml! {
			@active[app]
			$( @config [ $($config_data)* ] )?
			@app [
				icon = [ $($icon)* ]
				website = [ $($website)* ]
				authors = [ $($authors)* ]
				copyright = [ $($copyright)* ]
				name = [ $($name)* ]
				version = [ $($version)* ]
				description = [ $expr ]
			]

			$($all)*
		}
	};

	[ // END
		$( @active[$($_:tt)*] )?
		@config [
			qualifier = [ $($qualifier:tt)* ]
			organization = [ $($organization:tt)* ]
			file_name = [ $($file_name:tt)* ]
		]
		@app [
			icon = [ $($icon:tt)* ]
			website = [ $($website:tt)* ]
			authors = [ $($authors:tt)* ]
			copyright = [ $($copyright:tt)* ]
			name = [ $($name:tt)* ]
			version = [ $($version:tt)* ]
			description = [ $($description:tt)* ]
		]
	] => {
		// config
		pub const CONFIG_QUALIFIER: &str = $($qualifier)*;
		pub const CONFIG_ORGANIZATION: &str = $($organization)*;
		pub const CONFIG_FILE_NAME: &str = $($file_name)*;

		// app
		pub const APP_ID: &str = $crate::concat_str!(
			CONFIG_QUALIFIER, // com
			".",
			CONFIG_ORGANIZATION, // ulinkot
			".",
			APP_PKG_ICON // machinepmmeter
		);

		pub const APP_PKG_ICON: &str = $($icon)*;
		pub const APP_PKG_WEBSITE: &str = $($website)*;
		pub const APP_PKG_AUTHORS: &[&str] = &[$($authors)*];
		pub const APP_PKG_COPYRIGHT: &str = $($copyright)*;

		pub const APP_PKG_NAME: &str = $($name)*;
		pub const UPPERCASE_APP_PKG_NAME: &str = $crate::const_ascii_uppercase!(APP_PKG_NAME);

		pub const APP_PKG_VERSION: &str = $($version)*;
		pub const UPPERCASE_APP_PKG_VERSION: &str = $crate::const_ascii_uppercase!(APP_PKG_VERSION);

		pub const APP_PKG_DESCRIPTION: &str = $($description)*;

	};
}

pub(crate) use parse_and_impl_project_toml;
