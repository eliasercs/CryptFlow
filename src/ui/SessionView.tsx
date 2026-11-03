import type { AuthenticatedUser } from "../domain/auth";

interface SessionViewProps {
    user: AuthenticatedUser;
    error: string | null;
    onLogout: () => void;
}

/** Vista con la sesión iniciada. Aquí irán después el selector de archivos y el cifrado. */
export function SessionView({ user, error, onLogout }: SessionViewProps) {
    return (
        <main className="card">
            <h1>🔐 CryptFlow</h1>
            <p>
                Sesión: <strong>{user.username}</strong>
            </p>
            <button type="button" className="secondary" onClick={onLogout}>
                Cerrar sesión
            </button>
            {error && (
                <p role="alert" className="error">
                    {error}
                </p>
            )}
        </main>
    );
}