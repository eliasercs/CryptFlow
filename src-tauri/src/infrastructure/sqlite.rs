use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};

use rusqlite::{params, Connection, ErrorCode, OptionalExtension};

use crate::domain::error::DomainError;
use crate::domain::ports::UserRepository;
use crate::domain::user::{User, UserId, Username};

/// SQLite permite una conexión por hilo a la vez: se comparte detrás de un candado.
pub type SharedConnection = Arc<Mutex<Connection>>;

/// Crea las tablas si no existen. Se puede ejecutar todas las veces que haga falta.
pub fn migrate(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS users (
             id              INTEGER PRIMARY KEY AUTOINCREMENT,
             username        TEXT NOT NULL UNIQUE COLLATE NOCASE,
             password_hash   TEXT NOT NULL,
             failed_attempts INTEGER NOT NULL DEFAULT 0,
             locked_until    INTEGER NOT NULL DEFAULT 0,
             created_at      INTEGER NOT NULL
         );",
    )
}

pub fn open_database(path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA journal_mode = WAL;")?;
    migrate(&conn)?;
    Ok(conn)
}

fn db_err(e: rusqlite::Error) -> DomainError {
    DomainError::Internal(format!("base de datos: {e}"))
}

/// Adaptador del puerto `UserRepository` con SQLite.
pub struct SqliteUserRepository {
    conn: SharedConnection,
}

impl SqliteUserRepository {
    pub fn new(conn: SharedConnection) -> Self {
        Self { conn }
    }

    fn lock(&self) -> Result<MutexGuard<'_, Connection>, DomainError> {
        self.conn
            .lock()
            .map_err(|_| DomainError::Internal("conexión de base de datos no disponible".into()))
    }
}

impl UserRepository for SqliteUserRepository {
    fn find_by_username(&self, username: &Username) -> Result<Option<User>, DomainError> {
        let conn = self.lock()?;
        let row = conn
            .query_row(
                "SELECT id, username, password_hash, failed_attempts, locked_until
                 FROM users WHERE username = ?1",
                [username.as_str()],
                |r| {
                    Ok((
                        r.get::<_, i64>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, i64>(3)?,
                        r.get::<_, i64>(4)?,
                    ))
                },
            )
            .optional()
            .map_err(db_err)?;

        row.map(|(id, name, hash, failed_attempts, locked_until)| {
            Ok(User {
                id: UserId(id),
                // Se vuelve a validar: no se confía en lo que haya en el disco.
                username: Username::parse(&name).map_err(|_| {
                    DomainError::Internal("nombre de usuario inválido en la base de datos".into())
                })?,
                password_hash: hash,
                failed_attempts: failed_attempts.max(0) as u32,
                locked_until,
            })
        })
        .transpose()
    }

    fn create(&self, username: &Username, password_hash: &str, now: i64) -> Result<User, DomainError> {
        let conn = self.lock()?;
        let inserted = conn.execute(
            "INSERT INTO users (username, password_hash, created_at) VALUES (?1, ?2, ?3)",
            params![username.as_str(), password_hash, now],
        );
        match inserted {
            Ok(_) => Ok(User {
                id: UserId(conn.last_insert_rowid()),
                username: username.clone(),
                password_hash: password_hash.to_string(),
                failed_attempts: 0,
                locked_until: 0,
            }),
            // La restricción UNIQUE cubre la carrera entre "comprobar" e "insertar".
            Err(rusqlite::Error::SqliteFailure(e, _)) if e.code == ErrorCode::ConstraintViolation => {
                Err(DomainError::UsernameTaken)
            }
            Err(e) => Err(db_err(e)),
        }
    }

    fn save_lock_state(&self, user: &User) -> Result<(), DomainError> {
        let conn = self.lock()?;
        conn.execute(
            "UPDATE users SET failed_attempts = ?2, locked_until = ?3 WHERE id = ?1",
            params![user.id.0, user.failed_attempts, user.locked_until],
        )
        .map(|_| ())
        .map_err(db_err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::test_support::assert_user_repository_contract;

    fn in_memory_repo() -> SqliteUserRepository {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        SqliteUserRepository::new(Arc::new(Mutex::new(conn)))
    }

    #[test]
    fn sqlite_repository_meets_the_repository_contract() {
        assert_user_repository_contract(&in_memory_repo());
    }

    #[test]
    fn migrate_can_run_twice() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        migrate(&conn).unwrap();
    }

    #[test]
    fn data_survives_closing_and_reopening_the_file() {
        let path = std::env::temp_dir().join(format!("cryptflow-test-{}.db", std::process::id()));
        let ana = Username::parse("ana").unwrap();

        {
            let conn = open_database(&path).unwrap();
            let repo = SqliteUserRepository::new(Arc::new(Mutex::new(conn)));
            repo.create(&ana, "hash", 1).unwrap();
        } // aquí se cierra la conexión

        let conn = open_database(&path).unwrap();
        let repo = SqliteUserRepository::new(Arc::new(Mutex::new(conn)));
        assert!(repo.find_by_username(&ana).unwrap().is_some());

        drop(repo);
        for suffix in ["", "-wal", "-shm"] {
            let _ = std::fs::remove_file(format!("{}{suffix}", path.display()));
        }
    }
}