use gtk::prelude::*;
use gtk::{Align, BoxBuilder, Orientation, Switch};
use gtk::{Application, ApplicationWindowBuilder};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

// ANCHOR: activate
fn build_ui(application: &Application) {
    // Create a window
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();

    // ANCHOR: switch
    // Create the switch
    let switch = Switch::new();

    // Set and then immediately obtain state
    switch.set_state(true);
    let current_state = switch.state();

    // This prints: "The current state is true"
    println!("The current state is {}", current_state);
    // ANCHOR_END: switch

    // Set up box
    let gtk_box = BoxBuilder::new()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&switch);
    window.set_child(Some(&gtk_box));
    window.present();
}
// ANCHOR_END: activate
