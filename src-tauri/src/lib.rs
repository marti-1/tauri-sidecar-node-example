
use tauri_plugin_shell::ShellExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            
            let sidecar_command = app.shell().sidecar("sidecar").unwrap();
            let (mut _rx, _child) = sidecar_command.spawn().unwrap();

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
