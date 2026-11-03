import { afterEach, describe, expect, it } from "vitest";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";

import { AuthError } from "../../domain/auth";
import { TauriAuthService } from "./TauriAuthService";

afterEach(() => clearMocks());

/** Simula el backend: registra cada llamada y responde con `reply`. */
function fakeBackend(reply: unknown) {
    const calls: Array<{ command: string; args: unknown }> = [];
    mockIPC((command, args) => {
        calls.push({ command, args });
        if (typeof reply === "function") return reply(command);
        return reply;
    });
    return calls;
}

describe("TauriAuthService", () => {
    it("register envía usuario y contraseña al comando y devuelve el usuario", async () => {
        const calls = fakeBackend("ana");

        const user = await new TauriAuthService().register("ana", "clave-larga-1");

        expect(user).toEqual({ username: "ana" });
        expect(calls).toEqual([
            { command: "register", args: { username: "ana", password: "clave-larga-1" } },
        ]);
    });

    it("login llama al comando login", async () => {
        const calls = fakeBackend("ana");

        const user = await new TauriAuthService().login("ana", "clave-larga-1");

        expect(user).toEqual({ username: "ana" });
        expect(calls[0].command).toBe("login");
    });

    it("currentUser devuelve null cuando no hay sesión y el usuario cuando la hay", async () => {
        fakeBackend(null);
        expect(await new TauriAuthService().currentUser()).toBeNull();

        fakeBackend("ana");
        expect(await new TauriAuthService().currentUser()).toEqual({ username: "ana" });
    });

    it("logout llama al comando logout", async () => {
        const calls = fakeBackend(undefined);
        await new TauriAuthService().logout();
        expect(calls[0].command).toBe("logout");
    });

    it("convierte el mensaje de error del backend en un AuthError", async () => {
        fakeBackend(() => {
            throw "Usuario o contraseña incorrectos";
        });

        const attempt = new TauriAuthService().login("ana", "mala");

        await expect(attempt).rejects.toBeInstanceOf(AuthError);
        await expect(attempt).rejects.toThrow("Usuario o contraseña incorrectos");
    });

    it("usa un mensaje genérico si el error no es un texto", async () => {
        fakeBackend(() => {
            throw { raro: true };
        });

        await expect(new TauriAuthService().logout()).rejects.toThrow(
            "Ocurrió un error inesperado",
        );
    });
});