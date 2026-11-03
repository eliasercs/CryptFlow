import { useAuth } from "../application/useAuth";
import { AuthScreen } from "./AuthScreen";
import { SessionView } from "./SessionView";

/** Decide qué pantalla mostrar según el estado de la autenticación. */
export function App() {
    const auth = useAuth();

    if (auth.loading) {
        return <p className="center">Cargando…</p>;
    }

    if (auth.user) {
        return <SessionView user={auth.user} error={auth.error} onLogout={auth.logout} />;
    }

    return (
        <AuthScreen
            busy={auth.busy}
            error={auth.error}
            onLogin={auth.login}
            onRegister={auth.register}
            onModeChange={auth.clearError}
        />
    );
}