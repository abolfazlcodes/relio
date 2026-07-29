#![cfg(feature = "desktop-runtime")]

use std::sync::{Arc, Mutex};
use std::thread;

use tauri::{Emitter, Manager};
use uuid::Uuid;

use crate::lifecycle::{DisplayBounds, LaunchIntent, ShutdownCoordinator, WindowGeometry};
use crate::local_metadata::LifecycleMetadataStore;
use crate::single_instance::{self, SingleInstance};

pub fn run() {
    if let Err(error) = run_inner() {
        eprintln!("Relio could not start: {error}");
        std::process::exit(1);
    }
}

fn run_inner() -> Result<(), Box<dyn std::error::Error>> {
    let runtime_directory = single_instance::default_runtime_directory()?;
    let instance = single_instance::acquire(&runtime_directory)?;
    let SingleInstance::Primary(primary) = instance else {
        let SingleInstance::Secondary(endpoint) = instance else {
            unreachable!("single-instance result is exhaustive");
        };
        endpoint.forward(Uuid::now_v7(), LaunchIntent::Activate)?;
        return Ok(());
    };

    let metadata_store = Arc::new(LifecycleMetadataStore::open(
        runtime_directory.join("metadata"),
    )?);
    let previous_metadata = metadata_store.mark_startup_unclean()?;
    let shutdown = Arc::new(Mutex::new(ShutdownCoordinator::default()));
    let last_geometry = Arc::new(Mutex::new(previous_metadata.window.clone().map(Into::into)));

    let application = tauri::Builder::default()
        .setup({
            let shutdown = Arc::clone(&shutdown);
            move |app| {
                if let (Some(saved), Some(window)) =
                    (previous_metadata.window, app.get_webview_window("main"))
                {
                    restore_window(&window, saved.into());
                }

                let handle = app.handle().clone();
                thread::Builder::new()
                    .name("relio-launch-endpoint".into())
                    .spawn(move || {
                        loop {
                            match primary.endpoint().receive_once() {
                                Ok(message) => {
                                    if let Some(window) = handle.get_webview_window("main") {
                                        let _ = window.show();
                                        let _ = window.unminimize();
                                        let _ = window.set_focus();
                                    }
                                    if let LaunchIntent::OpenWorkspace { workspace_id } =
                                        message.intent
                                    {
                                        let _ = handle
                                            .emit("relio://launch/open-workspace", workspace_id);
                                    }
                                }
                                Err(error) => {
                                    eprintln!("Rejected local launch intent: {error}");
                                }
                            }
                        }
                    })?;
                app.manage(shutdown);
                Ok(())
            }
        })
        .build(tauri::generate_context!())?;

    application.run(move |handle, event| match event {
        tauri::RunEvent::WindowEvent {
            label,
            event: tauri::WindowEvent::CloseRequested { .. },
            ..
        } if label == "main" => {
            if let Some(window) = handle.get_webview_window("main") {
                if let Ok(mut geometry) = last_geometry.lock() {
                    *geometry = capture_window(&window);
                }
            }
            if let Ok(mut coordinator) = shutdown.lock() {
                let _ = coordinator.begin_unblocked_shutdown();
            }
        }
        tauri::RunEvent::WindowEvent {
            label,
            event: tauri::WindowEvent::Destroyed,
            ..
        } if label == "main" => {
            if let Ok(mut coordinator) = shutdown.lock() {
                coordinator.on_webview_lost();
            }
        }
        tauri::RunEvent::Exit => {
            let geometry = last_geometry.lock().ok().and_then(|value| *value);
            if let Err(error) = metadata_store.mark_clean_exit(geometry) {
                eprintln!("Could not persist clean shutdown metadata: {error}");
            }
        }
        _ => {}
    });
    Ok(())
}

fn restore_window(window: &tauri::WebviewWindow, saved: WindowGeometry) {
    let displays = window
        .available_monitors()
        .unwrap_or_default()
        .into_iter()
        .map(|monitor| {
            let position = monitor.position();
            let size = monitor.size();
            DisplayBounds {
                x: position.x,
                y: position.y,
                width: size.width,
                height: size.height,
            }
        })
        .collect::<Vec<_>>();
    let fallback = WindowGeometry {
        x: 80,
        y: 80,
        width: 1120,
        height: 720,
        maximized: false,
    };
    let safe = saved.restore(&displays, fallback);
    let _ = window.set_position(tauri::PhysicalPosition::new(safe.x, safe.y));
    let _ = window.set_size(tauri::PhysicalSize::new(safe.width, safe.height));
    if safe.maximized {
        let _ = window.maximize();
    }
}

fn capture_window(window: &tauri::WebviewWindow) -> Option<WindowGeometry> {
    let position = window.outer_position().ok()?;
    let size = window.inner_size().ok()?;
    Some(WindowGeometry {
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
        maximized: window.is_maximized().unwrap_or(false),
    })
}
