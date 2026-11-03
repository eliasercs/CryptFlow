import type { ReactNode } from "react";
import { renderHook, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { AuthService } from "../domain/auth";
import { ServicesProvider, useServices, type Services } from "./services";

/** Un servicio de autenticación mínimo: lo justo para cumplir la interfaz. */
function stubAuth(label: string): AuthService {
    return {
        register: async () => ({ username: label }),
        login: async () => ({ username: label }),
        logout: async () => { },
        currentUser: async () => ({ username: label }),
    };
}

function providerFor(services: Services) {
    return function Wrapper({ children }: { children: ReactNode }) {
        return <ServicesProvider services={services}>{children}</ServicesProvider>;
    };
}

describe("ServicesProvider / useServices", () => {
    it("entrega a los componentes los servicios que recibió el proveedor", () => {
        const services: Services = { auth: stubAuth("real") };

        const { result } = renderHook(() => useServices(), { wrapper: providerFor(services) });

        expect(result.current.auth).toBe(services.auth);
    });

    it("cada proveedor entrega sus propios servicios (así se inyectan dobles de prueba)", async () => {
        function WhoAmI() {
            const { auth } = useServices();
            return <button onClick={() => void auth.currentUser()}>listo</button>;
        }

        const a: Services = { auth: stubAuth("A") };
        const b: Services = { auth: stubAuth("B") };
        const spyA = vi.spyOn(a.auth, "currentUser");
        const spyB = vi.spyOn(b.auth, "currentUser");

        render(
            <ServicesProvider services={a}>
                <WhoAmI />
            </ServicesProvider>,
        );
        screen.getByText("listo").click();

        expect(spyA).toHaveBeenCalledTimes(1);
        expect(spyB).not.toHaveBeenCalled();
    });

    it("falla con un mensaje claro si se usa fuera del proveedor", () => {
        // React escribe el error en la consola; se silencia para que la salida quede limpia.
        const silenced = vi.spyOn(console, "error").mockImplementation(() => { });

        expect(() => renderHook(() => useServices())).toThrow("ServicesProvider");

        silenced.mockRestore();
    });
});