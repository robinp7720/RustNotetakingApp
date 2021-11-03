use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::main_window;

#[derive(Debug, Default)]
pub struct TestApplication {}

#[glib::object_subclass]
impl ObjectSubclass for TestApplication {
    const NAME: &'static str = "TestApplication";
    type Type = super::TestApplication;
    type ParentType = gtk::Application;
}

impl ObjectImpl for TestApplication {}

impl ApplicationImpl for TestApplication {
    fn activate(&self, application: &Self::Type) {
        let window = main_window::MainWindow::new(application);

        window.show();
    }
}

impl GtkApplicationImpl for TestApplication {}
