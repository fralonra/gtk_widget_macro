# gtk_widget_macro

[![Latest version](https://img.shields.io/crates/v/gtk_widget_macro.svg)](https://crates.io/crates/gtk_widget_macro)
[![Documentation](https://docs.rs/gtk_widget_macro/badge.svg)](https://docs.rs/gtk_widget_macro)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

A derive macro helps you handle Gtk widgets.

## How it works

`gtk_widget_macro` exports a derive macro named `GtkWidget`.

When `GtkWidget` is added to a struct, it defines a `Struct::from_builder(&gtk::Builder) -> Struct` function to that struct.

When this function is called, it collects all the fields of the struct and uses each field's name as a sole parameter passed to `gtk::Builder.object(glib::gstring::IntoGStr)`.
The retrieved object will be assigned to the corresponding field.
If the object with the given name is not found, the process panics.
Besides, a new method named exactly the field's name will be added to the struct.
This method will return a reference of that object.

This macro is Gtk-version agnostic. You can use it with whatever Gtk version you like. Report a bug if there are some versions do not work well.

## Exmaple

```rust
use gtk::{prelude::*, Application, ApplicationWindow, Builder, Button};
// Imports the macro.
use gtk_widget_macro::GtkWidget;

// Adds the macro to the struct.
#[derive(GtkWidget)]
struct Widgets {
    button_example: Button,
    window: ApplicationWindow,
}

fn build_ui(app: &Application) {
    let builder = Builder::from_string(include_str!("main.ui"));

    // Calls `from_builder` to construct the struct.
    let widgets = Widgets::from_builder(builder);

    // Retrieves the gtk::Button with object id `button_example`.
    let button_example = widgets.button_example();
    button_example.connect_clicked(|_| {
        println!("Button clicked.");
    });

    // Retrieves the gtk::ApplicationWindow with object id `window`.
    let window = widgets.window();
    window.set_application(Some(app));
    window.present(); // Needs to be changed to `show_all` if Gtk3 is used.
}

fn main() {
    let app = Application::builder().application_id("org.example.Example").build();

    app.connect_activate(build_ui);

    app.run();
}
```
