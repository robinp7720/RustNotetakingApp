use glib::clone;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*, CompositeTemplate, GestureStylus};

use crate::test_application::widgets::canvas;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "main_window.ui")]
pub struct MainWindow {
    pub dialog: gtk::FileChooserNative,
    #[template_child(id = "testLabel")]
    pub test_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub canvas: TemplateChild<canvas::CustomCanvasWidget>,
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    /*fn new() -> Self {
        let dialog = gtk::FileChooserNative::new(
            Some("Open File"),
            gtk::NONE_WINDOW,
            gtk::FileChooserAction::Open,
            Some("Open"),
            Some("Cancel"),
        );

        dialog.set_modal(true);

        Self {
            dialog,
            test_label: TemplateChild::default(),
            //stylus_handler_id,
        }
    }*/

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        //super.painter_parent.set_child(Some(&picture));

        klass.install_action(
            "openDialog",
            None,
            move |win, action_name, action_target| {
                println!("{:?}, {:?}", action_name, action_target);
                let self_ = MainWindow::from_instance(win);
                self_.dialog.set_transient_for(Some(win));

                self_
                    .dialog
                    .connect_response(clone!(@weak win => move |d, response| {
                        if response == gtk::ResponseType::Accept {}

                        d.destroy();
                    }));

                self_.dialog.show();
            },
        );

        klass.install_action(
            "testButton",
            None,
            move |win, action_name, action_target| {
                let self_ = MainWindow::from_instance(win);
                println!("{:?}, {:?}", action_name, action_target);
            },
        );
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn constructed(&self, obj: &Self::Type) {
        let hello_world = GestureStylus::new();

        hello_world.connect_down(move |event, x, y| {
            if let Some(test) = event.device_tool() {
                println!(
                    "Down: {}, {}, {}, {}",
                    x,
                    y,
                    test.hardware_id(),
                    test.tool_type()
                );
            }
        });

        hello_world.connect_motion(move |event, x, y| {
            if let Some(test) = event.device_tool() {
                //println!("Motion: {}, {}, {}", x, y, test.hardware_id());
            }
        });
        hello_world.connect_up(move |event, x, y| {
            if let Some(test) = event.device_tool() {
                println!(
                    "Up: {}, {}, {}, {}",
                    x,
                    y,
                    test.hardware_id(),
                    test.tool_type()
                );
            } else {
                println!("Up: {}, {}", x, y);
            }
        });
        hello_world.connect_proximity(move |event, x, y| {
            //println!("Prox: {}, {}", x, y);
        });
        obj.add_controller(&hello_world);
    }
}

impl WidgetImpl for MainWindow {}
impl WindowImpl for MainWindow {}

impl ApplicationWindowImpl for MainWindow {}
