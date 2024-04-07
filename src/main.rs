use gtk::{glib::clone, prelude::*, glib};

const APP_ID: &str = "gtk.todo_list";
const DEFAULT_WINDOW_DIMENSIONS: (i32, i32) = (600, 800);
fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();

    app.connect_activate(app_on_activate);
    app.run();
}

fn app_on_activate(app: &gtk::Application) {
    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let button_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let emoji_chooser = gtk::EmojiChooser::builder()
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .margin_bottom(10)
        .build();

    emoji_chooser
        .connect_emoji_picked(|_emoji_chooser, picked| println!("User picked the emoji: {picked}"));

    let text = gtk::Label::builder().label("Todo List").build();
    let button = gtk::Button::builder().label("Emojis").build();

    button.connect_clicked(clone!(@strong emoji_chooser =>  move |_button| {
        emoji_chooser.set_visible(!emoji_chooser.get_visible())
    }));

    button_box.append(&button);
    button_box.append(&emoji_chooser);

    gtk_box.append(&text);
    gtk_box.append(&button_box);

    let window = gtk::Window::builder()
        .application(app)
        .title("Todo List")
        .child(&gtk_box)
        .default_width(DEFAULT_WINDOW_DIMENSIONS.0)
        .default_height(DEFAULT_WINDOW_DIMENSIONS.1)
        .build();

    window.present();
}
