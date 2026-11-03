use std::sync::Arc;

use tauri::State;

use super::errors::error_message;
use crate::application::auth::AuthUseCases;

#[tauri::command]
pub async fn register(
    auth: State<'_, Arc<AuthUseCases>>,
    username: String,
    password: String,
) -> Result<String, String> {
    let auth = auth.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        auth.register
            .execute(&username, &password)
            .map(|session| session.username.as_str().to_string())
            .map_err(|e| error_message(&e))
    })
    .await
    .map_err(|e| format!("Error interno: {e}"))?
}

#[tauri::command]
pub async fn login(
    auth: State<'_, Arc<AuthUseCases>>,
    username: String,
    password: String,
) -> Result<String, String> {
    let auth = auth.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        auth.login
            .execute(&username, &password)
            .map(|session| session.username.as_str().to_string())
            .map_err(|e| error_message(&e))
    })
    .await
    .map_err(|e| format!("Error interno: {e}"))?
}

#[tauri::command]
pub fn logout(auth: State<'_, Arc<AuthUseCases>>) {
    auth.logout.execute();
}

#[tauri::command]
pub fn current_user(auth: State<'_, Arc<AuthUseCases>>) -> Option<String> {
    auth.current_user
        .execute()
        .map(|session| session.username.as_str().to_string())
}