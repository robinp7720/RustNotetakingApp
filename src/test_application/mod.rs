mod imp;
mod main_window;
mod widgets;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct TestApplication(ObjectSubclass<imp::TestApplication>) @extends gio::Application, gtk::Application, @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for TestApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl TestApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &"me.robindecker.test"),
            ("flags", &gio::ApplicationFlags::empty()),
        ])
        .expect("Failed to create TestApplication")
    }
}
