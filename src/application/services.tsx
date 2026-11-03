import { createContext, useContext, type ReactNode } from "react";

import type { AuthService } from "../domain/auth";

/** Todo lo que la aplicación necesita del exterior, entregado como interfaces. */
export interface Services {
    auth: AuthService;
}

const ServicesContext = createContext<Services | null>(null);

interface ServicesProviderProps {
    services: Services;
    children: ReactNode;
}

/** Pone los servicios al alcance de todos los componentes que haya debajo. */
export function ServicesProvider({ services, children }: ServicesProviderProps) {
    return <ServicesContext.Provider value={services}>{children}</ServicesContext.Provider>;
}

/** Lee los servicios. Solo funciona dentro de un `ServicesProvider`. */
export function useServices(): Services {
    const services = useContext(ServicesContext);
    if (services === null) {
        throw new Error("useServices debe usarse dentro de <ServicesProvider>");
    }
    return services;
}