
use tauri_plugin_shell::ShellExt;

// system tray code
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_shell::process::CommandChild;
use std::sync::Mutex;

struct SidecarState(Mutex<Option<CommandChild>>);
// -- end system tray code

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let sidecar_command = app.shell().sidecar("sidecar").unwrap();
            let (_rx, child) = sidecar_command.spawn().unwrap();
            
            // system tray code
            app.manage(SidecarState(Mutex::new(Some(child))));

            let quit_i = MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            let _tray = TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        let state = app.state::<SidecarState>();
                        if let Some(child) = state.0.lock().unwrap().take() {
                            let _ = child.kill();
                        }
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;
            // -- end system tray code
            std::process::Command::new("google-chrome")
                // .arg("--new-window")
                .arg("http://localhost:3000")
                .spawn()
                .expect("Failed to open chrome process");
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
