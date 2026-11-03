import type { ReactNode } from "react";
import { act, renderHook, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import type { AuthService } from "../domain/auth";
import { FakeAuthService } from "./testing/FakeAuthService";
import { ServicesProvider } from "./services";
import { useAuth } from "./useAuth";

function setup(auth: AuthService) {
    const wrapper = ({ children }: { children: ReactNode }) => (
        <ServicesProvider services={{ auth }}>{children}</ServicesProvider>
    );
    return renderHook(() => useAuth(), { wrapper });
}

async function settled(auth: AuthService) {
    const hook = setup(auth);
    await waitFor(() => expect(hook.result.current.loading).toBe(false));
    return hook;
}

describe("useAuth", () => {
    it("empieza cargando y termina sin usuario si no hay sesión", async () => {
        const { result } = setup(new FakeAuthService());
        expect(result.current.loading).toBe(true);

        await waitFor(() => expect(result.current.loading).toBe(false));
        expect(result.current.user).toBeNull();
    });

    it("recupera la sesión que ya estaba abierta", async () => {
        const { result } = await settled(new FakeAuthService().seed("ana", "clave-larga-1", true));
        expect(result.current.user).toEqual({ username: "ana" });
    });

    it("login correcto deja al usuario autenticado", async () => {
        const { result } = await settled(new FakeAuthService().seed("ana", "clave-larga-1"));

        await act(async () => {
            await result.current.login("ana", "clave-larga-1");
        });

        expect(result.current.user).toEqual({ username: "ana" });
        expect(result.current.error).toBeNull();
        expect(result.current.busy).toBe(false);
    });

    it("login incorrecto muestra el error del servicio y no autentica", async () => {
        const { result } = await settled(new FakeAuthService().seed("ana", "clave-larga-1"));

        await act(async () => {
            await result.current.login("ana", "mala");
        });

        expect(result.current.user).toBeNull();
        expect(result.current.error).toBe("Usuario o contraseña incorrectos");
    });

    it("un intento nuevo borra el error anterior", async () => {
        const { result } = await settled(new FakeAuthService().seed("ana", "clave-larga-1"));

        await act(async () => {
            await result.current.login("ana", "mala");
        });
        expect(result.current.error).not.toBeNull();

        await act(async () => {
            await result.current.login("ana", "clave-larga-1");
        });
        expect(result.current.error).toBeNull();
    });

    it("registrarse abre la sesión y cerrar sesión la termina", async () => {
        const { result } = await settled(new FakeAuthService());

        await act(async () => {
            await result.current.register("luis", "clave-larga-1");
        });
        expect(result.current.user).toEqual({ username: "luis" });

        await act(async () => {
            await result.current.logout();
        });
        expect(result.current.user).toBeNull();
    });

    it("si no se puede consultar la sesión, queda sin usuario y con un error", async () => {
        const broken: AuthService = {
            register: async () => ({ username: "x" }),
            login: async () => ({ username: "x" }),
            logout: async () => { },
            currentUser: async () => {
                throw new Error("sin canal");
            },
        };

        const { result } = await settled(broken);

        expect(result.current.user).toBeNull();
        expect(result.current.error).toBe("Ocurrió un error inesperado");
    });
});