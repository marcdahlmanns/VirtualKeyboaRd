
use gtk::{prelude::*, ApplicationWindow};
use gtk::{glib, Application};

const APP_ID: &str = "virtual-keyboaRd";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window  = ApplicationWindow::builder()
        .application(app)
        .title("Virtual Keyboard")
        .build();
    window.present();
}
