mod custom_paintable;
mod imp;

use gtk::{gdk, glib};

glib::wrapper! {
    pub struct CustomCanvasWidget(ObjectSubclass<imp::CustomCanvasWidget>)
        @extends gtk::Widget;
}

impl Default for CustomCanvasWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomCanvasWidget {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create CustomCanvasWidget")
    }
}
