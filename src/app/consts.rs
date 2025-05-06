use crate::const_ascii_uppercase;

pub const APP_ID: &str = "com.ulinkot.machinepmmeter";
pub const APP_PKG_ICON: &str = env!("CARGO_PKG_NAME");
pub const APP_PKG_WEBSITE: &str = env!("CARGO_PKG_REPOSITORY");
pub const APP_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const APP_PKG_COPYRIGHT: &str = "© 2025 Denis Kotlyarov";

pub const APP_PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const UPPERCASE_APP_PKG_NAME: &str = const_ascii_uppercase!(APP_PKG_NAME);

pub const APP_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const UPPERCASE_APP_PKG_VERSION: &str = const_ascii_uppercase!(APP_PKG_VERSION);

pub const APP_PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
