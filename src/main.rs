mod widgets;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CenterBox, gdk, glib};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::collections::HashMap;
use tokio::runtime::Runtime;

use widgets::clock::ClockWidget;
use widgets::workspaces::WorkspacesWidget;

struct BarManager {
    clocks: HashMap<String, ClockWidget>,
    workspaces: HashMap<String, WorkspacesWidget>,
}

impl BarManager {
    fn new() -> Self {
        Self {
            clocks: HashMap::new(),
            workspaces: HashMap::new(),
        }
    }

    fn add_clock(&mut self, monitor_name: String, clock: ClockWidget) {
        self.clocks.insert(monitor_name, clock);
    }

    fn add_workspaces(&mut self, monitor_name: String, workspaces: WorkspacesWidget) {
        self.workspaces.insert(monitor_name, workspaces);
    }

    fn update_all_clocks(&self) {
        for clock in self.clocks.values() {
            clock.update();
        }
    }

    async fn update_all_workspaces(&self) {
        for workspaces in self.workspaces.values() {
            workspaces.update().await;
        }
    }
}

fn main() {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let _guard = rt.enter();

    let app = Application::builder()
        .application_id("com.github.sokolawesome.forgebar")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let display = gdk::Display::default().expect("Failed to get default display");
    let monitors = display.monitors();

    let mut bar_manager = BarManager::new();

    for i in 0..monitors.n_items() {
        if let Some(monitor) = monitors.item(i).and_downcast::<gdk::Monitor>() {
            let monitor_name = monitor
                .connector()
                .unwrap_or_else(|| format!("monitor-{}", i).into());
            let window = create_bar_window(app, &monitor, &monitor_name, &mut bar_manager);
            window.present();
        }
    }

    let bar_manager = std::rc::Rc::new(std::cell::RefCell::new(bar_manager));

    glib::timeout_add_seconds_local(1, move || {
        let bar_manager = bar_manager.clone();
        glib::MainContext::default().spawn_local(async move {
            bar_manager.borrow().update_all_clocks();
            bar_manager.borrow().update_all_workspaces().await;
        });
        glib::ControlFlow::Continue
    });
}

fn create_bar_window(
    app: &Application,
    monitor: &gdk::Monitor,
    monitor_name: &str,
    bar_manager: &mut BarManager,
) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(format!("forgebar - {}", monitor_name))
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_exclusive_zone(35);
    window.set_namespace(Some("forgebar"));
    window.set_monitor(Some(monitor));

    let main_box = CenterBox::new();
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);
    main_box.set_margin_top(5);
    main_box.set_margin_bottom(5);

    let left_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    let workspaces = WorkspacesWidget::new();
    workspaces.setup_css();
    left_box.append(&workspaces.widget());

    bar_manager.add_workspaces(monitor_name.to_string(), workspaces);

    let clock = ClockWidget::new();
    bar_manager.add_clock(monitor_name.to_string(), clock.clone());

    let right_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);

    main_box.set_start_widget(Some(&left_box));
    main_box.set_center_widget(Some(&clock.widget()));
    main_box.set_end_widget(Some(&right_box));

    window.set_child(Some(&main_box));

    window
}
