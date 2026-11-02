# CryptFlow

Es un software de escritorio utilizado para cifrar y descifrar archivos por medio de una clave, con acceso protegido mediante credencial de usuario almacenado en una base de datos local. Todo el proceso se desarrolla en tu equipo: sin nube y sin servidores. Este proyecto lo desarrollé con el propósito de proponer un mecanismo para proteger la integridad del contenido de mis archivos.

## Características

- Cifrado por archivo con clave propia. Cada archivo se cifra y descifra con la clave que indiques. Incluye un generador de claves aleatorias.
- Autenticación local. Para proteger el acceso al contenido del software se habilita un formulario de registro e inicio de sesión. Una vez que crees tu cuenta, puedes deshabilitar el registro de nuevas cuentas desde el interior del software.
- Navegación por directorio. Introduces la ruta de una carpeta para realizar el recorrido de su contenido y seleccionar el archivo a cifrar/descifrar.
- Barra de progreso con bytes procesados.
- Nombrado de archivos. Al cifrar, el archivo recibe un nombre aleatorio único, el nombre original se almacena de manera cifrada y se restaura al descifrar.
- Sin duplicación. Al terminar la operación, el archivo original se elimina.
- Historial de archivos manipulados con opción de borrarlo.

## Forma de uso

1. Crea una cuenta (usuario y contraseña de al menos 8 caracteres) o inicia sesión.
2. Escribe la ruta de un directorio y pulsa Abrir (o Examinar…). Entra a las carpetas con un clic y selecciona el archivo.
3. Escribe la clave del archivo, o pulsa Generar clave segura y guárdala.
4. Pulsa Cifrar o Descifrar y sigue la barra de progreso.

Al cifrar se solicita repetir la clave: como el original se elimina, un error de escritura en la clave dejaría el archivo irrecuperable.

> Si pierdes la clave, el archivo no se puede recuperar. No existe opción de restablecimiento.

## Requisitos

- Rust y Node JS 18 o superior
- Dependencias para Windows: WebView2 y BuildTools de Visual Studio.

## Instalación

```bash
npm install
npm run dev
npm run build
```

> Puedes descargar el instalador de las diferentes versiones de este software en la sección de Releases de este repositorio.