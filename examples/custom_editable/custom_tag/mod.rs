pub mod imp;

use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct CustomTag(ObjectSubclass<imp::CustomTag>) @extends gtk::Widget;
}

impl CustomTag {
    pub fn new(label: &str) -> Self {
        glib::Object::new(&[("label", &label), ("has-close-button", &true)])
            .expect("Failed to create a CustomTag")
    }

    pub fn set_has_close_button(&self, has_close_button: bool) {
        let self_ = imp::CustomTag::from_instance(self);
        if self_.has_close_button.get() == has_close_button {
            return;
        }

        if has_close_button {
            let button = gtk::ButtonBuilder::new()
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .has_frame(false)
                .build();
            button.connect_clicked(clone!(@weak self as tag => move |_btn| {
                tag.emit_by_name("closed", &[]).unwrap();
            }));
            let icon = gtk::Image::from_icon_name(Some("window-close-symbolic"));
            button.set_child(Some(&icon));

            self_.container.append(&button);
            self_.button.replace(Some(button));
        } else if let Some(button) = self_.button.borrow_mut().take() {
            self_.container.remove(&button);
        }
        self_.has_close_button.set(has_close_button);
    }
}
