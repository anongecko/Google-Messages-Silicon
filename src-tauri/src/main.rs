// This is a macOS-only application, so the windows_subsystem attribute is not needed.

use tauri::{
    menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu},
    AppHandle, Manager, Runtime, WindowEvent,
};

fn main() {
    tauri::Builder::default()
        // Initialize the opener plugin. Navigation is now handled by tauri.conf.json.
        .plugin(tauri_plugin_opener::init())
        .menu(|handle| create_menu(handle))
        .setup(|app| {
            // Setup is only used to show the window. Navigation is handled declaratively.
            if let Some(main_window) = app.get_webview_window("main") {
                main_window.show()?;
            }
            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Helper function to create the application menu, using the correct Tauri v2.6.2 API.
fn create_menu<R: Runtime>(handle: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let app_name = &handle.package_info().name;

    // --- App Menu ---
    // The correct pattern is to use `Submenu::with_items` to build the submenu and its contents at once.
    let app_menu = Submenu::with_items(
        handle,
        app_name,
        true,
        &[
            &PredefinedMenuItem::about(
                handle,
                None,
                Some(AboutMetadata {
                    name: Some(app_name.to_string()),
                    ..Default::default()
                }),
            )?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::services(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::hide(handle, None)?,
            &PredefinedMenuItem::hide_others(handle, None)?,
            &PredefinedMenuItem::show_all(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::quit(handle, None)?,
        ],
    )?;

    // --- File Menu ---
    let file_menu = Submenu::with_items(
        handle,
        "File",
        true,
        &[&PredefinedMenuItem::close_window(handle, None)?],
    )?;

    // --- Edit Menu ---
    let edit_menu = Submenu::with_items(
        handle,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(handle, None)?,
            &PredefinedMenuItem::redo(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::cut(handle, None)?,
            &PredefinedMenuItem::copy(handle, None)?,
            &PredefinedMenuItem::paste(handle, None)?,
            &PredefinedMenuItem::select_all(handle, None)?,
        ],
    )?;

    // --- Window Menu ---
    let window_menu = Submenu::with_items(
        handle,
        "Window",
        true,
        &[&PredefinedMenuItem::minimize(handle, None)?],
    )?;

    // --- Assemble the Full Menu ---
    // The correct pattern is to create a mutable menu and append the submenus to it.
    let mut menu = Menu::new(handle)?;
    menu.append(&app_menu)?;
    menu.append(&file_menu)?;
    menu.append(&edit_menu)?;
    menu.append(&window_menu)?;

    Ok(menu)
}

