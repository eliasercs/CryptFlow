import { AuthError, type AuthenticatedUser, type AuthService } from "../../domain/auth";

/** Servicio de autenticación falso, en memoria, para probar hooks y pantallas. */
export class FakeAuthService implements AuthService {
    private readonly users = new Map<string, string>();
    private session: AuthenticatedUser | null = null;

    /** Deja un usuario ya registrado (y opcionalmente con la sesión abierta). */
    seed(username: string, password: string, signedIn = false): this {
        this.users.set(username.toLowerCase(), password);
        if (signedIn) this.session = { username };
        return this;
    }

    async register(username: string, password: string): Promise<AuthenticatedUser> {
        if (this.users.has(username.toLowerCase())) {
            throw new AuthError("Ese nombre de usuario ya está en uso");
        }
        this.users.set(username.toLowerCase(), password);
        this.session = { username };
        return this.session;
    }

    async login(username: string, password: string): Promise<AuthenticatedUser> {
        if (this.users.get(username.toLowerCase()) !== password) {
            throw new AuthError("Usuario o contraseña incorrectos");
        }
        this.session = { username };
        return this.session;
    }

    async logout(): Promise<void> {
        this.session = null;
    }

    async currentUser(): Promise<AuthenticatedUser | null> {
        return this.session;
    }
}