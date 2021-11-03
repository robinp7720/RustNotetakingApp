use gtk::glib;
use gtk::prelude::{DrawingAreaExt, DrawingAreaExtManual, FrameExt, WidgetExt};
use gtk::subclass::prelude::*;
use std::cell::RefCell;

use rand::Rng;

use crate::test_application::widgets::canvas::custom_paintable::CustomPaintable;

#[derive(Debug, Default)]
pub struct CustomCanvasWidget {
    area: RefCell<Option<gtk::DrawingArea>>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomCanvasWidget {
    const NAME: &'static str = "CustomCanvasWidget";
    type Type = super::CustomCanvasWidget;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();

        //klass.set_css_name("button");
    }
}

impl ObjectImpl for CustomCanvasWidget {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        /*let paintable = CustomPaintable::new();
        let picture = gtk::Picture::new();
        picture.set_halign(gtk::Align::Center);
        picture.set_size_request(obj.width(), obj.height());
        picture.set_paintable(Some(&paintable));
        picture.set_parent(obj);*/
        //

        //let frame = gtk::Frame::new(None);

        //frame.set_vexpand(true);
        //frame.set_hexpand(true);

        let area = gtk::DrawingArea::new();

        area.set_content_width(200);
        area.set_content_height(200);

        let mut rng = RefCell::new(rand::thread_rng());

        // let clo = move ||{rng.gen_range(0.0, 1.0)};
        area.set_draw_func(move |area, context, a, b| {
            println!("w: {} c:{}", a as f64, b as f64);
            let r = rng.borrow_mut().gen_range(0.0, 1.0);
            let g = rng.borrow_mut().gen_range(0.0, 1.0);
            let b = rng.borrow_mut().gen_range(0.0, 1.0);
            context.set_source_rgb(r, g, b);
            context.rectangle(0f64, 0f64, a as f64, 200f64);
            context.fill().expect("uh oh");
            //gtk::Inhibit(false)
        });

        area.set_parent(obj);

        *self.area.borrow_mut() = Some(area);

        let gesture = gtk::GestureClick::new();

        gesture.connect_released(|gesture, _, _, _| {});

        obj.add_controller(&gesture);
    }

    fn dispose(&self, _obj: &Self::Type) {
        if let Some(area) = self.area.borrow_mut().take() {
            area.unparent();
        }
    }
}

impl WidgetImpl for CustomCanvasWidget {}
