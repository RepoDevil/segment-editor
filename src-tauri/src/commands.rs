#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri::Manager;

#[tauri::command]
pub async fn show_main_window(window: tauri::Window) {
#[cfg(not(any(target_os = "android", target_os = "ios")))]
    window.get_webview_window("main").unwrap().show().unwrap();
}
