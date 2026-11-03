/** Persona con la sesión iniciada. */
export interface AuthenticatedUser {
    readonly username: string;
}

/** Error de autenticación con un mensaje listo para mostrar. */
export class AuthError extends Error {
    constructor(message: string) {
        super(message);
        this.name = "AuthError";
    }
}

/** Lo que la interfaz necesita de la autenticación, sin saber cómo se hace. */
export interface AuthService {
    register(username: string, password: string): Promise<AuthenticatedUser>;
    login(username: string, password: string): Promise<AuthenticatedUser>;
    logout(): Promise<void>;
    currentUser(): Promise<AuthenticatedUser | null>;
}