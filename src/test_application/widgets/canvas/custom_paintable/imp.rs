use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene, gsk};

#[derive(Default)]
pub struct CustomPaintable {}

#[glib::object_subclass]
impl ObjectSubclass for CustomPaintable {
    const NAME: &'static str = "CustomPaintable";
    type Type = super::CustomPaintable;
    type ParentType = glib::Object;
    type Interfaces = (gdk::Paintable,);
}

impl ObjectImpl for CustomPaintable {}

impl PaintableImpl for CustomPaintable {
    fn flags(&self, _paintable: &Self::Type) -> gdk::PaintableFlags {
        // Fixed size
        gdk::PaintableFlags::SIZE
    }

    fn intrinsic_width(&self, _paintable: &Self::Type) -> i32 {
        0
    }

    fn intrinsic_height(&self, _paintable: &Self::Type) -> i32 {
        0
    }

    fn snapshot(&self, _paintable: &Self::Type, snapshot: &gdk::Snapshot, width: f64, height: f64) {
        let snapshot = snapshot.downcast_ref::<gtk::Snapshot>().unwrap();
        // Draw a linear gradient
        snapshot.append_linear_gradient(
            &graphene::Rect::new(0_f32, 0_f32, width as f32, height as f32),
            &graphene::Point::new(0f32, 0f32),
            &graphene::Point::new(width as f32, height as f32),
            &[
                gsk::ColorStop::new(0.0, gdk::RGBA::red()),
                gsk::ColorStop::new(
                    0.15,
                    gdk::RGBA {
                        red: 0f32,
                        green: 0f32,
                        blue: 0f32,
                        alpha: 0f32,
                    },
                ),
                gsk::ColorStop::new(
                    0.3,
                    gdk::RGBA {
                        red: 0f32,
                        green: 0f32,
                        blue: 0f32,
                        alpha: 0f32,
                    },
                ),
                gsk::ColorStop::new(0.45, gdk::RGBA::green()),
                gsk::ColorStop::new(0.6, gdk::RGBA::blue()),
                gsk::ColorStop::new(
                    0.75,
                    gdk::RGBA {
                        red: 0f32,
                        green: 0f32,
                        blue: 0f32,
                        alpha: 0f32,
                    },
                ),
                gsk::ColorStop::new(
                    0.9,
                    gdk::RGBA {
                        red: 0f32,
                        green: 0f32,
                        blue: 0f32,
                        alpha: 0f32,
                    },
                ),
            ],
        );
    }
}
