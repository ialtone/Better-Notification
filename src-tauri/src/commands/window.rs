use tauri::Window;

#[tauri::command]
pub fn close_window(window: Window) {
    window.close().unwrap();
}
