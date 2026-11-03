#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::{Arc, Mutex};

use tauri::Manager;

use application::auth::AuthUseCases;
use infrastructure::argon2_hasher::Argon2PasswordHasher;
use infrastructure::memory_session::InMemorySessionStore;
use infrastructure::sqlite::{open_database, SqliteUserRepository};
use infrastructure::system_clock::SystemClock;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Dónde guardar los datos: carpeta propia de la app en el sistema.
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let connection = Arc::new(Mutex::new(open_database(&data_dir.join("cryptflow.db"))?));

            // Se enchufan los adaptadores concretos a los casos de uso.
            let auth = AuthUseCases::new(
                Arc::new(SqliteUserRepository::new(connection)),
                Arc::new(Argon2PasswordHasher),
                Arc::new(SystemClock),
                Arc::new(InMemorySessionStore::default()),
            );
            app.manage(Arc::new(auth));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            presentation::auth_commands::register,
            presentation::auth_commands::login,
            presentation::auth_commands::logout,
            presentation::auth_commands::current_user,
        ])
        .run(tauri::generate_context!())
        .expect("error al iniciar la aplicación");
}