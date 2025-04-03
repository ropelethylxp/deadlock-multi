use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label};

pub struct GUI {
    app: Application,
}

impl GUI {
    pub fn new() -> Self {
        let app = Application::new(Some("com.example.deadlock_multi"), Default::default());
        GUI { app }
    }

    pub fn run(&self) {
        self.app.connect_activate(|app| {
            let window = ApplicationWindow::new(app);
            window.set_title("Deadlock Multi");
            window.set_default_size(400, 200);

            let label = Label::new(Some("Welcome to Deadlock Multi"));
            let button = Button::with_label("Launch Game");

            button.connect_clicked(|_| {
                // Launch game logic
            });

            let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
            vbox.pack_start(&label, true, true, 0);
            vbox.pack_start(&button, true, true, 0);
            window.add(&vbox);
            window.show_all();
        });

        self.app.run();
    }
}