use chrono::Local;
use gtk4::Label;

#[derive(Clone)]
pub struct ClockWidget {
    label: Label,
}

impl ClockWidget {
    pub fn new() -> Self {
        let label = Label::new(None);
        label.set_markup("<span font='16'>Loading...</span>");

        Self { label }
    }

    pub fn widget(&self) -> Label {
        self.label.clone()
    }

    pub fn update(&self) {
        let now = Local::now();
        let formatted_time = now.format("%H:%M:%S | %d.%m.%y").to_string();
        self.label
            .set_markup(&format!("<span font='16'>{}</span>", formatted_time));
    }
}
