use glib;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Orientation};
use serde::Deserialize;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

#[derive(Debug, Deserialize)]
struct Workspace {
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
struct WorkspaceEvent {
    #[serde(rename = "activeWorkspace")]
    active_workspace: Workspace,
}

#[derive(Clone)]
pub struct WorkspacesWidget {
    container: GtkBox,
    buttons: HashMap<i32, Button>,
    active_workspace: std::rc::Rc<std::cell::RefCell<i32>>,
}

impl WorkspacesWidget {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 5);
        let mut buttons = HashMap::new();
        let active_workspace = std::rc::Rc::new(std::cell::RefCell::new(1));

        for i in 1..=8 {
            let button = Button::with_label(&i.to_string());
            button.set_size_request(30, 25);

            let workspace_id = i;
            button.connect_clicked(move |_| {
                glib::spawn_future_local(async move {
                    if let Err(e) = switch_workspace(workspace_id).await {
                        eprintln!("error: failed to switch workspace: {}", e);
                    }
                });
            });

            container.append(&button);
            buttons.insert(i, button);
        }

        Self {
            container,
            buttons,
            active_workspace,
        }
    }

    pub fn widget(&self) -> GtkBox {
        self.container.clone()
    }

    pub async fn update(&self) {
        if let Ok(active_id) = get_active_workspace().await {
            let old_active = *self.active_workspace.borrow();
            *self.active_workspace.borrow_mut() = active_id;

            if let Some(old_button) = self.buttons.get(&old_active) {
                old_button.set_css_classes(&[]);
            }

            if let Some(new_button) = self.buttons.get(&active_id) {
                new_button.set_css_classes(&["active-workspace"]);
            }
        }
    }

    pub fn setup_css(&self) {
        let css_provider = gtk4::CssProvider::new();
        css_provider.load_from_data(
            "
            .active-workspace {
                color: #ff0000;
                font-weight: bold;
                text-decoration: underline;
            }
            button {
                background: none;
                border: none;
                color: #d8dee9;
                font-size: 16px;
                padding: 0 5px;
            }
            button:hover {
                color: #eceff4;
            }
        ",
        );

        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

async fn get_active_workspace() -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
    let socket_path = get_hyprland_socket_path()?;
    let mut stream = UnixStream::connect(socket_path).await?;

    stream.write_all(b"j/activeworkspace").await?;
    stream.flush().await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    let workspace: Workspace = serde_json::from_str(&response)?;
    Ok(workspace.id)
}

async fn switch_workspace(
    workspace_id: i32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let socket_path = get_hyprland_socket_path()?;
    let mut stream = UnixStream::connect(socket_path).await?;

    let command = format!("dispatch workspace {}", workspace_id);
    stream.write_all(command.as_bytes()).await?;

    Ok(())
}

fn get_hyprland_socket_path() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let instance_signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .map_err(|_| "HYPRLAND_INSTANCE_SIGNATURE not found")?;

    Ok(format!(
        "/run/user/1000/hypr/{}/.socket.sock",
        instance_signature
    ))
}
