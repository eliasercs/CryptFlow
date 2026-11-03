import { useCallback, useEffect, useState } from "react";

import { AuthError, type AuthenticatedUser } from "../domain/auth";
import { useServices } from "./services";

function messageOf(error: unknown): string {
    return error instanceof AuthError ? error.message : "Ocurrió un error inesperado";
}

/** Estado y acciones de autenticación para la interfaz. */
export function useAuth() {
    const { auth } = useServices();
    const [user, setUser] = useState<AuthenticatedUser | null>(null);
    const [loading, setLoading] = useState(true);
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState<string | null>(null);

    // Al montar: ¿ya hay una sesión abierta?
    useEffect(() => {
        let active = true;
        auth
            .currentUser()
            .then((current) => {
                if (active) setUser(current);
            })
            .catch((e: unknown) => {
                if (active) setError(messageOf(e));
            })
            .finally(() => {
                if (active) setLoading(false);
            });
        return () => {
            active = false;
        };
    }, [auth]);

    const authenticate = useCallback(async (action: () => Promise<AuthenticatedUser>) => {
        setBusy(true);
        setError(null);
        try {
            setUser(await action());
        } catch (e) {
            setError(messageOf(e));
        } finally {
            setBusy(false);
        }
    }, []);

    const login = useCallback(
        (username: string, password: string) => authenticate(() => auth.login(username, password)),
        [auth, authenticate],
    );

    const register = useCallback(
        (username: string, password: string) => authenticate(() => auth.register(username, password)),
        [auth, authenticate],
    );

    const logout = useCallback(async () => {
        try {
            await auth.logout();
            setUser(null);
            setError(null);
        } catch (e) {
            setError(messageOf(e));
        }
    }, [auth]);

    const clearError = useCallback(() => setError(null), []);

    return { user, loading, busy, error, login, register, logout, clearError };
}