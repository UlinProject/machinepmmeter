use crate::__gen_transparent_gtk_type;
use crate::app::config::FontAppConfig;
use crate::app::dockwindow::AppViDockWindow;
use crate::app::dockwindow::PosINScreen;
use crate::core::display::ViGraphDisplayInfo;
use crate::widgets::primitives::label::ViLabel;
use enclose::enc;
use gtk::Align;
use gtk::Box;
use gtk::Notebook;
use gtk::ScrolledWindow;
use gtk::ffi::GtkNotebook;
use gtk::glib::Cast;
use gtk::pango::Weight;
use gtk::pango::WrapMode;
use gtk::prelude::NotebookExtManual;
use gtk::traits::BinExt;
use gtk::traits::BoxExt;
use gtk::traits::ContainerExt;
use gtk::traits::NotebookExt;
use gtk::traits::ScrolledWindowExt;
use gtk::traits::StyleContextExt;
use gtk::traits::WidgetExt;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[repr(transparent)]
#[derive(Debug)]
pub struct ViNotebook(Notebook);

__gen_transparent_gtk_type! {
	#[sys(GtkNotebook)]
	ViNotebook(
		new |a: Notebook| {
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

impl ViNotebook {
	pub fn new(
		c_display: &Rc<ViGraphDisplayInfo>,
		dock_window: &AppViDockWindow,
		pos_inscreen: &Rc<RefCell<PosINScreen>>,
	) -> Self {
		let notebook = Notebook::new();
		notebook.style_context().add_class("vinotebook");
		notebook.connect_switch_page(
			enc!((dock_window, c_display, pos_inscreen) move |notebook, _page, page_num| {
				for i in 0..notebook.n_pages() {
					if let Some(child) = notebook.nth_page(Some(i)) {
						if let Some(tab_label) = notebook.tab_label(&child) {
							if let Some(label) = tab_label.downcast_ref::<ViLabel>() {
								let style = label.style_context();

								if i == page_num {
									if !style.has_class("active_head_vinotebook") {
										style.add_class("active_head_vinotebook");

										label.read_text(|text| {
											label.set_text(&format!("# {}", text));
										});

										if let Ok(scrolled_window) = child.downcast::<ScrolledWindow>() {
											let height = if let Some(child) = scrolled_window.child() {
												let (m, _) = child.preferred_size();
												m.height
											} else {
												0
											};

											scrolled_window.set_hexpand(true);
											scrolled_window.set_vexpand(true);
											scrolled_window.set_size_request(-1, height);

											scrolled_window.set_max_content_height(i32::MAX);
										}
									}
									continue;
								}

								if style.has_class("active_head_vinotebook") {
									style.remove_class("active_head_vinotebook");

									label.read_text(|text| {
										if let Some(next_text) = text.strip_prefix("# ") {
											label.set_text(next_text);
										}
									});

									if let Ok(scrolled_window) = child.downcast::<ScrolledWindow>() {
										scrolled_window.set_hexpand(false);
										scrolled_window.set_vexpand(false);
										scrolled_window.set_size_request(-1, 1);
										scrolled_window.set_max_content_height(1);
									}
								}
							}
						}
					}
				}

				if let Some((window_width, height_window)) = dock_window.adjust_window_height() {
					dock_window.set_pos_inscreen(&*c_display, window_width, height_window, *RefCell::borrow(&pos_inscreen));
				}
			}),
		);
		notebook.set_visible(true);

		Self(notebook)
	}

	pub fn append_page(
		&self,
		f_app_config: impl AsRef<FontAppConfig>,
		tab_label: &str,
		notice: Option<&str>,
		append: impl FnOnce(&Box),
	) {
		let vbox = Box::new(gtk::Orientation::Vertical, 0);
		vbox.style_context().add_class("vinotebookpage");
		vbox.set_valign(gtk::Align::Fill);
		vbox.set_halign(gtk::Align::Baseline);

		append(&vbox);

		if let Some(notice) = notice {
			vbox.pack_end(
				&ViLabel::new((), &f_app_config, notice, Weight::Bold)
					.set_margin_start(3)
					.set_margin_end(3)
					.set_margin_bottom(3)
					.set_wrap(true)
					.set_wrap_mode(WrapMode::Word)
					.set_max_width_chars(45)
					.set_align(Align::Center)
					.connect_nonblack_background(0.0, 0.0, 0.0, 1.0),
				false,
				false,
				0,
			);
		}
		vbox.set_visible(true);

		let scrolled_window = ScrolledWindow::builder().build();
		scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic); // Disable horizontal scrolling
		scrolled_window.set_hexpand(false);
		scrolled_window.set_vexpand(false);
		scrolled_window.set_size_request(-1, 1);
		scrolled_window.set_max_content_height(1);

		scrolled_window.set_child(Some(&vbox));
		scrolled_window.set_visible(true);

		let n_page = self.0.append_page(
			&scrolled_window,
			Some(&ViLabel::new(
				"head_vinotebook",
				f_app_config,
				tab_label,
				Weight::Bold,
			)),
		);

		match n_page {
			0 => {
				if let Some(child) = self.0.nth_page(Some(0)) {
					if let Some(tab_label) = self.0.tab_label(&child) {
						if let Some(label) = tab_label.downcast_ref::<ViLabel>() {
							label.style_context().add_class("first_head_vinotebook");
						}
					}
				}
				for i in 1..self.0.n_pages() {
					if let Some(child) = self.0.nth_page(Some(i)) {
						if let Some(tab_label) = self.0.tab_label(&child) {
							if let Some(label) = tab_label.downcast_ref::<ViLabel>() {
								let style = label.style_context();
								if style.has_class("first_head_vinotebook") {
									style.remove_class("first_head_vinotebook");
								}
							}
						}
					}
				}
			}
			_ => {
				for i in 1..self.0.n_pages() {
					if let Some(child) = self.0.nth_page(Some(i)) {
						if let Some(tab_label) = self.0.tab_label(&child) {
							if let Some(label) = tab_label.downcast_ref::<ViLabel>() {
								let style = label.style_context();
								if !style.has_class("notfirst_head_vinotebook") {
									style.add_class("notfirst_head_vinotebook");
								}
							}
						}
					}
				}
			}
		}
	}
}

impl Deref for ViNotebook {
	type Target = Notebook;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
