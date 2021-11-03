mod test_application;

use test_application::TestApplication;

use gtk::prelude::*;

fn main() {
    let app = TestApplication::new();
    std::process::exit(app.run());
}
