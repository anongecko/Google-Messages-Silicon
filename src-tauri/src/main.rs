#[cfg(target_os = "macos")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use tauri::{
    menu::{AboutMetadata, Menu, PredefinedMenuItem, Submenu},
    AppHandle, Manager, Runtime, WindowEvent,
};

const APP_NAME: &str = "Gemini";

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .menu(|handle| create_menu(handle))
        .setup(|app| {
            if let Some(main_window) = app.get_webview_window("main") {
                // Small delay for initial render
                std::thread::sleep(std::time::Duration::from_millis(50));
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

fn create_menu<R: Runtime>(handle: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let app_menu = Submenu::with_items(
        handle,
        APP_NAME,
        true,
        &[
            &PredefinedMenuItem::about(
                handle,
                None,
                Some(AboutMetadata {
                    name: Some(APP_NAME.to_string()),
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

    let file_menu = Submenu::with_items(
        handle,
        "File",
        true,
        &[&PredefinedMenuItem::close_window(handle, None)?],
    )?;

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

    let window_menu = Submenu::with_items(
        handle,
        "Window",
        true,
        &[&PredefinedMenuItem::minimize(handle, None)?],
    )?;

    let menu = Menu::new(handle)?;
    menu.append(&app_menu)?;
    menu.append(&file_menu)?;
    menu.append(&edit_menu)?;
    menu.append(&window_menu)?;

    Ok(menu)
}
