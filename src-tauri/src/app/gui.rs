use std::fs;

use local_ip_address::local_ip;
use tauri::{utils::config::AppUrl, Manager, WindowUrl};
use window_shadows::set_shadow;

use crate::{
    python::{self, PythonRuntime},
    LAUNCHER_DIRECTORY,
};

use super::app_data::Config;

#[derive(serde::Serialize)]
struct ServerState {
    host: String,
    port: u16,
}

#[derive(serde::Serialize)]
struct ShareResponse {
    url: String,
    host: String,
    port: u16,
}

#[tauri::command]
async fn share_url(state: tauri::State<'_, ServerState>) -> Result<ShareResponse, String> {
    Ok(ShareResponse {
        url: format!("http://{}:{}", state.host, state.port),
        host: state.host.clone(),
        port: state.port,
    })
}

#[tauri::command]
async fn run_server() -> Result<(), String> {
    let data = LAUNCHER_DIRECTORY.data_dir();
    let runtimes_folder = data.parent().unwrap().join("runtimes");
    let python = python::download_python(&runtimes_folder)
        .await
        .expect("Failed to download Python");
    let runtime = PythonRuntime::new(python);
    runtime
        .execute(
            vec![
                "-m".to_string(),
                "pip".to_string(),
                "install".to_string(),
                "--upgrade".to_string(),
                "pip".to_string(),
            ],
            &data,
        )
        .await
        .expect("Failed to upgrade pip");
    runtime
        .handle_io(
            &mut runtime
                .execute(
                    vec![
                        "-m".to_string(),
                        "pip".to_string(),
                        "install".to_string(),
                        "git+https://github.com/OMUCHAT/server.git".to_string(),
                    ],
                    &data,
                )
                .await
                .expect("Failed to start server"),
            |_, data| {
                println!("{}", String::from_utf8_lossy(data));
                Ok(())
            },
            |_, data| {
                println!("{}", String::from_utf8_lossy(data));
                Ok(())
            },
            tokio::sync::oneshot::channel().1,
            &(),
        )
        .await
        .expect("Failed to install server");
    runtime
        .handle_io(
            &mut runtime
                .execute(vec!["-m".to_string(), "omuserver".to_string()], &data)
                .await
                .expect("Failed to start server"),
            |_, data| {
                println!("{}", String::from_utf8_lossy(data));
                Ok(())
            },
            |_, data| {
                println!("{}", String::from_utf8_lossy(data));
                Ok(())
            },
            tokio::sync::oneshot::channel().1,
            &(),
        )
        .await
        .expect("Failed to handle IO");

    Ok(())
}

#[tauri::command]
async fn delete_runtime() -> Result<(), String> {
    let runtimes_folder = std::path::PathBuf::from("runtimes");
    fs::remove_dir_all(&runtimes_folder).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn gui_main() {
    let mut context = tauri::generate_context!();

    let host: std::net::IpAddr = local_ip().expect("failed to get local IP");
    let port = if cfg!(dev) {
        5173u16
    } else {
        portpicker::pick_unused_port().expect("failed to find unused port")
    };

    let url = format!("http://{}:{}", host, port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    // rewrite the config so the IPC is enabled on this URL
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());

    tauri::Builder::default()
        .manage(ServerState {
            host: host.to_string(),
            port,
        })
        .plugin(omuchat_tauri_plugin_server::Builder::new(port).build())
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            share_url,
            run_server,
            delete_runtime,
        ])
        // run async run_server() in a background thread
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
