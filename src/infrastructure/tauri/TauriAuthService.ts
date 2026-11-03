import { invoke } from "@tauri-apps/api/core";

import {
    AuthError,
    type AuthenticatedUser,
    type AuthService,
} from "../../domain/auth";

/** Llama a un comando de Rust y convierte cualquier rechazo en un `AuthError`. */
async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
    try {
        return await invoke<T>(command, args);
    } catch (error) {
        throw new AuthError(
            typeof error === "string" ? error : "Ocurrió un error inesperado",
        );
    }
}

/** Adaptador del puerto `AuthService` sobre los comandos de Tauri. */
export class TauriAuthService implements AuthService {
    async register(username: string, password: string): Promise<AuthenticatedUser> {
        const name = await call<string>("register", { username, password });
        return { username: name };
    }

    async login(username: string, password: string): Promise<AuthenticatedUser> {
        const name = await call<string>("login", { username, password });
        return { username: name };
    }

    async logout(): Promise<void> {
        await call<void>("logout");
    }

    async currentUser(): Promise<AuthenticatedUser | null> {
        const name = await call<string | null>("current_user");
        return name === null ? null : { username: name };
    }
}