import { afterEach } from "vitest";
import { cleanup } from "@testing-library/react";

// Desmonta lo que cada prueba haya dibujado, para que no se mezclen entre sí.
afterEach(() => cleanup());