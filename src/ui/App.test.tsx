import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { ServicesProvider } from "../application/services";
import { FakeAuthService } from "../application/testing/FakeAuthService";
import { App } from "./App";

function renderApp(auth: FakeAuthService) {
    render(
        <ServicesProvider services={{ auth }}>
            <App />
        </ServicesProvider>,
    );
}

describe("App", () => {
    it("sin sesión muestra la pantalla de acceso", async () => {
        renderApp(new FakeAuthService());
        expect(screen.getByText("Cargando…")).toBeTruthy();
        expect(await screen.findByRole("button", { name: "Entrar" })).toBeTruthy();
    });

    it("con la sesión ya abierta entra directo", async () => {
        renderApp(new FakeAuthService().seed("ana", "clave-larga-1", true));
        expect(await screen.findByText("ana")).toBeTruthy();
        expect(screen.getByRole("button", { name: "Cerrar sesión" })).toBeTruthy();
    });

    it("flujo completo: iniciar sesión, ver la sesión y cerrarla", async () => {
        renderApp(new FakeAuthService().seed("ana", "clave-larga-1"));

        fireEvent.change(await screen.findByLabelText("Usuario"), { target: { value: "ana" } });
        fireEvent.change(screen.getByLabelText("Contraseña"), { target: { value: "clave-larga-1" } });
        fireEvent.click(screen.getByRole("button", { name: "Entrar" }));

        expect(await screen.findByText("ana")).toBeTruthy();

        fireEvent.click(screen.getByRole("button", { name: "Cerrar sesión" }));
        expect(await screen.findByRole("button", { name: "Entrar" })).toBeTruthy();
    });

    it("con credenciales incorrectas muestra el error y sigue en el acceso", async () => {
        renderApp(new FakeAuthService().seed("ana", "clave-larga-1"));

        fireEvent.change(await screen.findByLabelText("Usuario"), { target: { value: "ana" } });
        fireEvent.change(screen.getByLabelText("Contraseña"), { target: { value: "mala" } });
        fireEvent.click(screen.getByRole("button", { name: "Entrar" }));

        expect((await screen.findByRole("alert")).textContent).toBe("Usuario o contraseña incorrectos");
    });
});