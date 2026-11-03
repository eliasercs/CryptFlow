import { useState, type FormEvent } from "react";

type Mode = "login" | "register";

interface AuthScreenProps {
    busy: boolean;
    error: string | null;
    onLogin: (username: string, password: string) => void;
    onRegister: (username: string, password: string) => void;
    onModeChange: () => void;
}

/** Pantalla de acceso: solo dibuja y avisa lo que la persona hizo. */
export function AuthScreen({ busy, error, onLogin, onRegister, onModeChange }: AuthScreenProps) {
    const [mode, setMode] = useState<Mode>("login");
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [confirm, setConfirm] = useState("");
    const [localError, setLocalError] = useState<string | null>(null);

    function switchTo(next: Mode) {
        setMode(next);
        setLocalError(null);
        onModeChange();
    }

    function submit(event: FormEvent) {
        event.preventDefault();
        if (mode === "register") {
            if (password !== confirm) {
                setLocalError("Las contraseñas no coinciden");
                return;
            }
            setLocalError(null);
            onRegister(username, password);
        } else {
            onLogin(username, password);
        }
    }

    const shownError = localError ?? error;
    const submitLabel = busy ? "Verificando…" : mode === "login" ? "Entrar" : "Crear cuenta";

    return (
        <main className="card">
            <h1>🔐 CryptFlow</h1>

            <div className="tabs">
                <button type="button" className={mode === "login" ? "tab active" : "tab"} onClick={() => switchTo("login")}>
                    Iniciar sesión
                </button>
                <button type="button" className={mode === "register" ? "tab active" : "tab"} onClick={() => switchTo("register")}>
                    Crear cuenta
                </button>
            </div>

            <form onSubmit={submit}>
                <label>
                    Usuario
                    <input value={username} onChange={(e) => setUsername(e.target.value)} autoComplete="username" />
                </label>
                <label>
                    Contraseña
                    <input
                        type="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                        autoComplete={mode === "login" ? "current-password" : "new-password"}
                    />
                </label>
                {mode === "register" && (
                    <label>
                        Repetir contraseña
                        <input
                            type="password"
                            value={confirm}
                            onChange={(e) => setConfirm(e.target.value)}
                            autoComplete="new-password"
                        />
                    </label>
                )}

                <button type="submit" className="primary" disabled={busy}>
                    {submitLabel}
                </button>

                {shownError && (
                    <p role="alert" className="error">
                        {shownError}
                    </p>
                )}
            </form>
        </main>
    );
}