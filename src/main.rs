mod keyboard;
mod input;

use anyhow::{Context, Result};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gdk::prelude::*;
use keyboard::VirtualKeyboard;

const APP_ID: &str = "org.virtualkeyboard.app";

fn main() -> Result<()> {
    // Initialize GTK
    gtk::init().context("Failed to initialize GTK")?;

    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(|app| {
        if let Err(e) = build_ui(app) {
            eprintln!("Failed to build UI: {}", e);
            std::process::exit(1);
        }
    });

    // Run the application
    app.run();

    Ok(())
}

fn build_ui(app: &Application) -> Result<()> {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Virtual Keyboard")
        .default_width(800)
        .default_height(300)
        .resizable(false)
        .focusable(false)
        .focus_on_click(false)
        .build();

    // Prevent the window from accepting focus
    window.set_can_focus(false);
    window.set_focus_on_click(false);

    // Create the virtual keyboard
    let keyboard = VirtualKeyboard::new();

    // Add the keyboard grid to the window
    window.set_child(Some(keyboard.get_grid()));

    // Present window
    window.present();

    // Set window properties to prevent focus stealing
    let surface = window.surface();
    if let Some(toplevel) = surface.downcast_ref::<gdk::Toplevel>() {
        // Request the window to not accept focus
        toplevel.focus(0); // timestamp 0 means don't focus
    }

    Ok(())
}
