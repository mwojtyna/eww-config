use hyprland::{
    data::Workspaces,
    event_listener::EventListener,
    shared::{HyprData, WorkspaceId},
};
use serde::Serialize;

#[derive(Serialize)]
struct WorkspaceData {
    id: WorkspaceId,
    occupied: bool,
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
        .iter()
        .map(|w| WorkspaceData {
            id: w.id,
            occupied: w.windows > 0,
        })
        .collect::<Vec<WorkspaceData>>();

    for i in 1..=10 {
        if !workspaces.iter().any(|w| w.id == i) {
            workspaces.push(WorkspaceData {
                id: i,
                occupied: false,
            });
        }
    }
    workspaces.sort_by(|a, b| a.id.cmp(&b.id));

    let json = serde_json::to_string(&workspaces).expect("Error serializing workspaces to json");
    println!("{}", json);
}
