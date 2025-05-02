use std::ops::Deref;

use crate::__gen_transparent_gtk_type;
use crate::PKG_AUTHORS;
use crate::PKG_COPYRIGHT;
use crate::PKG_DESCRIPTION;
use crate::PKG_NAME;
use crate::PKG_VERSION;
use crate::PKG_WEBSITE;
use gtk::AboutDialog;
use gtk::ffi::GtkAboutDialog;
use gtk::traits::AboutDialogExt;
use gtk::traits::DialogExt;
use gtk::traits::GtkWindowExt;

#[repr(transparent)]
#[derive(Debug)]
pub struct AppAboutDialog(AboutDialog);

__gen_transparent_gtk_type! {
	#[sys(GtkAboutDialog)]
	AppAboutDialog(
		new |a: AboutDialog| {
			Self(a)
		},
		ref |sself| {
			&sself.0
		},
		into |sself| {
			sself.0
		},
	)
}

impl Deref for AppAboutDialog {
	type Target = AboutDialog;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl AppAboutDialog {
	pub fn new(close_event: impl Fn() + 'static) -> Self {
		let about_dialog = AboutDialog::new();

		about_dialog.set_program_name(PKG_NAME);
		about_dialog.set_version(Some(PKG_VERSION));
		about_dialog.set_authors(&[PKG_AUTHORS]);
		about_dialog.set_copyright(Some(PKG_COPYRIGHT));
		about_dialog.set_comments(Some(PKG_DESCRIPTION));
		about_dialog.set_license_type(gtk::License::Gpl30);
		about_dialog.set_website_label(Some("Repository"));
		about_dialog.set_website(Some(PKG_WEBSITE));

		about_dialog.connect_response(move |dialog, _| {
			dialog.close();
			close_event();
		});

		Self(about_dialog)
	}
}
