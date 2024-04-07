use gtk::{
    gdk::Display,
    glib::{self, clone},
    prelude::*,
    CssProvider,
};

const APP_ID: &str = "gtk.todo_list";
const DEFAULT_WINDOW_DIMENSIONS: (i32, i32) = (600, 800);
fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(app_on_activate);
    app.run();
}

fn app_on_activate(app: &gtk::Application) {
    let content_body = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let task_div = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let title = gtk::Label::builder()
        .label("Todo List")
        .margin_bottom(20)
        .build();
    title.add_css_class("title_text");

    let add_item_textbox = gtk::Text::builder().margin_end(5).build();
    add_item_textbox.add_css_class("add_item_textbox");
    let add_item_button = gtk::Button::builder().label("Add Item").build();

    add_item_button.connect_clicked(clone!(@weak task_div, @weak add_item_textbox =>  move |_button| {
        let new_task_text = add_item_textbox.text();
        let new_task = gtk::Label::builder().label(new_task_text).margin_top(10).build();
        task_div.append(&new_task);
    }));

    let new_task_div = gtk::Grid::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .build();

    new_task_div.add_css_class("new_task");
    new_task_div.set_halign(gtk::Align::Center);

    const ADD_ITEM_TEXTBOX_WIDTH: i32 = 200;
    new_task_div.attach(&add_item_textbox, 0, 0, ADD_ITEM_TEXTBOX_WIDTH, 1);
    new_task_div.attach(&add_item_button, ADD_ITEM_TEXTBOX_WIDTH + 1, 0, 40, 1);

    let new_task_divider = gtk::Separator::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    content_body.append(&title);
    content_body.append(&new_task_div);
    content_body.append(&new_task_divider);
    content_body.append(&task_div);

    let window = gtk::Window::builder()
        .application(app)
        .title("Todo List")
        .child(&content_body)
        .default_width(DEFAULT_WINDOW_DIMENSIONS.0)
        .default_height(DEFAULT_WINDOW_DIMENSIONS.1)
        .build();

    window.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("css/styles.css"));
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
