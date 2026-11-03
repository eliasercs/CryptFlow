import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { AuthScreen } from "./AuthScreen";

function setup(overrides: Partial<Parameters<typeof AuthScreen>[0]> = {}) {
    const props = {
        busy: false,
        error: null,
        onLogin: vi.fn(),
        onRegister: vi.fn(),
        onModeChange: vi.fn(),
        ...overrides,
    };
    render(<AuthScreen {...props} />);
    return props;
}

function type(label: string, value: string) {
    fireEvent.change(screen.getByLabelText(label), { target: { value } });
}

describe("AuthScreen", () => {
    it("en modo iniciar sesión envía usuario y contraseña", () => {
        const props = setup();
        type("Usuario", "ana");
        type("Contraseña", "clave-larga-1");
        fireEvent.click(screen.getByRole("button", { name: "Entrar" }));

        expect(props.onLogin).toHaveBeenCalledWith("ana", "clave-larga-1");
        expect(props.onRegister).not.toHaveBeenCalled();
    });

    it("en modo crear cuenta exige que las contraseñas coincidan", () => {
        const props = setup();
        fireEvent.click(screen.getByRole("button", { name: "Crear cuenta" }));
        type("Usuario", "ana");
        type("Contraseña", "clave-larga-1");
        type("Repetir contraseña", "otra-distinta");
        fireEvent.click(screen.getAllByRole("button", { name: "Crear cuenta" })[1]);

        expect(screen.getByRole("alert").textContent).toBe("Las contraseñas no coinciden");
        expect(props.onRegister).not.toHaveBeenCalled();
    });

    it("en modo crear cuenta envía los datos cuando coinciden", () => {
        const props = setup();
        fireEvent.click(screen.getByRole("button", { name: "Crear cuenta" }));
        type("Usuario", "ana");
        type("Contraseña", "clave-larga-1");
        type("Repetir contraseña", "clave-larga-1");
        fireEvent.click(screen.getAllByRole("button", { name: "Crear cuenta" })[1]);

        expect(props.onRegister).toHaveBeenCalledWith("ana", "clave-larga-1");
        expect(props.onModeChange).toHaveBeenCalledTimes(1);
    });

    it("mientras trabaja deshabilita el botón y dice Verificando…", () => {
        setup({ busy: true });
        const button = screen.getByRole("button", { name: "Verificando…" });
        expect((button as HTMLButtonElement).disabled).toBe(true);
    });

    it("muestra el error que llega del exterior", () => {
        setup({ error: "Usuario o contraseña incorrectos" });
        expect(screen.getByRole("alert").textContent).toBe("Usuario o contraseña incorrectos");
    });
});