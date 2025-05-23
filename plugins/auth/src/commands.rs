use crate::{store::StoreKey, vault::VaultKey, AuthPluginExt};

#[tauri::command]
#[specta::specta]
pub fn start_oauth_server<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<u16, String> {
    app.start_oauth_server().map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn stop_oauth_server<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    port: u16,
) -> Result<(), String> {
    app.stop_oauth_server(port).map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn init_vault<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    user_id: String,
) -> Result<(), String> {
    app.init_vault(user_id).map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn reset_vault<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    app.reset_vault().map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn get_from_vault<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    key: VaultKey,
) -> Result<Option<String>, String> {
    app.get_from_vault(key).map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn set_in_vault<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    key: VaultKey,
    value: String,
) -> Result<(), String> {
    app.set_in_vault(key, value).map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn set_in_store<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    key: StoreKey,
    value: String,
) -> Result<(), String> {
    app.set_in_store(key, value).map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn get_from_store<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    key: StoreKey,
) -> Result<Option<String>, String> {
    app.get_from_store(key).map_err(|e| e.to_string())
}
