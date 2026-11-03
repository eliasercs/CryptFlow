use std::sync::{Mutex, MutexGuard};

use crate::domain::ports::SessionStore;
use crate::domain::session::Session;

/// Adaptador del puerto `SessionStore`: la sesión vive en memoria mientras la app está abierta.
#[derive(Default)]
pub struct InMemorySessionStore(Mutex<Option<Session>>);

impl InMemorySessionStore {
    fn guard(&self) -> MutexGuard<'_, Option<Session>> {
        // Si otro hilo falló con el candado tomado, el dato sigue siendo válido: se recupera.
        self.0.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl SessionStore for InMemorySessionStore {
    fn set(&self, session: Session) {
        *self.guard() = Some(session);
    }

    fn current(&self) -> Option<Session> {
        self.guard().clone()
    }

    fn clear(&self) {
        *self.guard() = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::{UserId, Username};
    use std::sync::Arc;

    fn session() -> Session {
        Session { user_id: UserId(7), username: Username::parse("ana").unwrap() }
    }

    #[test]
    fn stores_and_clears_the_session() {
        let store = InMemorySessionStore::default();
        assert_eq!(store.current(), None);

        store.set(session());
        assert_eq!(store.current(), Some(session()));

        store.clear();
        assert_eq!(store.current(), None);
    }

    #[test]
    fn can_be_shared_across_threads_as_a_port() {
        let store: Arc<dyn SessionStore> = Arc::new(InMemorySessionStore::default());
        let writer = store.clone();
        std::thread::spawn(move || writer.set(session())).join().unwrap();
        assert_eq!(store.current(), Some(session()));
    }

    #[test]
    fn survives_a_poisoned_lock() {
        let store = Arc::new(InMemorySessionStore::default());

        // Un hilo falla (panic) con el candado tomado: el candado queda "envenenado".
        let broken = store.clone();
        let result = std::thread::spawn(move || {
            let _guard = broken.0.lock().unwrap();
            panic!("falla simulada con el candado tomado");
        })
        .join();
        assert!(result.is_err());

        // Aun así, la sesión sigue funcionando.
        store.set(session());
        assert_eq!(store.current(), Some(session()));
    }
}