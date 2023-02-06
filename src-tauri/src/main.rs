#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{Manager, WindowBuilder};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.close().unwrap();
            // #[cfg(debug_assertions)]
            // main_window.open_devtools();
            WindowBuilder::new(
                app,
                "main-window".to_string(),
                tauri::WindowUrl::App("index.html".into())
            )
            .title("孔大夫的格式转换器")
            
            .on_web_resource_request(|req, resp| {
                if req.uri().starts_with("tauri://") {
                    resp.headers_mut().insert(
                        "Cross-Origin-Opener-Policy",
                        "same-origin".try_into().unwrap(),
                    );
                    resp.headers_mut().insert(
                        "Cross-Origin-Embedder-Policy",
                        "require-corp".try_into().unwrap(),
                    );
                    println!("done");
                }
            })
            .build()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
