use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label};
use std::process::Command;

fn main() {
    let app = Application::new(Some("com.example.deadlock_multi"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Deadlock Multi");
        window.set_default_size(400, 200);

        let label = Label::new(Some("Welcome to Deadlock Multi"));
        let button = Button::with_label("Launch Game");

        button.connect_clicked(|_| {
            Command::new("path/to/game.exe").spawn().expect("Failed to launch game");
        });

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&button, true, true, 0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}