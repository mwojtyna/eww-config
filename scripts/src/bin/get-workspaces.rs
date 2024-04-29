use hyprland::{
    data::Workspaces,
    event_listener::EventListener,
    shared::{HyprData, WorkspaceId},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WorkspaceData {
    id: WorkspaceId,
    windows: u16,
}

fn main() {
    update_workspaces();

    let mut listener = EventListener::new();
    listener.add_active_window_change_handler(|_| update_workspaces());
    listener.start_listener().expect("Failed to start listener");
}

fn update_workspaces() {
    let mut workspaces = Workspaces::get()
        .expect("Failed to get workspaces")
        .map(|w| WorkspaceData {
            id: w.id,
            windows: w.windows,
        })
        .collect::<Vec<WorkspaceData>>();

    for i in 1..=10 {
        if !workspaces.iter().any(|w| w.id == i) {
            workspaces.push(WorkspaceData { id: i, windows: 0 });
        }
    }
    workspaces.sort_by(|a, b| a.id.cmp(&b.id));

    let json = serde_json::to_string(&workspaces).expect("Error serializing workspaces to json");
    println!("{}", json);
}
