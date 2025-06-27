use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

fn main() {
    let app = Application::builder()
        .application_id("com.github.sokolawesome.forgebar")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("forgebar")
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Top);

    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);

    window.set_namespace(Some("forgebar"));

    let main_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);
    main_box.set_margin_top(5);
    main_box.set_margin_bottom(5);

    window.set_child(Some(&main_box));
    window.present();
}
