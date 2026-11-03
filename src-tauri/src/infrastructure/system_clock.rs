use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::ports::Clock;

/// Adaptador del puerto `Clock` con la hora real del sistema.
pub struct SystemClock;

impl Clock for SystemClock {
    /// Segundos desde 1970-01-01 (UTC).
    fn now(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_a_plausible_unix_timestamp() {
        // 1_700_000_000 es noviembre de 2023: cualquier hora real es posterior.
        assert!(SystemClock.now() > 1_700_000_000);
    }
}