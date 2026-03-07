# 💻 Registro de laptop


Proyecto desarrollado en Rust + Anchor para registrar información básica de una laptop
por ejemplo:(Marca, modelo, procesador,generacion y año).

La idea es tener un registro seguro, inmutable y descentralizado del registro laptops
para que no se pueda borrar.

# 🛠️ Herramientas

- **Lenguaje**: Rust.
- **Framework**: Anchor.
- **Entorno**: Solana Playground.

# 📂 Archivos
- **lib.rs**: Codigo del proyecto.
- **Registro-laptop.md**: Explicacion tecnica de como funciona el codigo (CRUD + PDA).

# 📌 Instrucciones

1. **Inicializar el sistema de registro**
- **Ejecutar la función inicializar_registro para crear el espacio de almacenamiento en
la blockchain.**

- **Semillas (Seeds): Se requiere la cadena exacta "registro" y la publicKey del administrador (owner).**

2. **Registrar una nueva laptop**
- **Usar la función agregar_laptop para dar de alta un equipo en el inventario.**
- **Datos requeridos:** Se deben ingresar la marca, modelo, procesador, número de serie y el año del equipo.





