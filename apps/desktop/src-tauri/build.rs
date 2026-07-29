fn main() {
    if std::env::var_os("CARGO_FEATURE_DESKTOP_RUNTIME").is_some() {
        tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
            tauri_build::AppManifest::new().commands(&[
                "terminal_list_shell_profiles",
                "terminal_start_local",
                "terminal_grant_output_credit",
                "terminal_send_input",
                "terminal_resize",
                "terminal_stop",
            ]),
        ))
        .expect("failed to build the explicit Relio desktop manifest");
    }
}
