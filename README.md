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

3. **Actualizar y Gestionar Componentes**
- **Utilizar actualizar_laptop para modificar el procesador de un equipo existente (buscando por su modelo).**
- **Usar eliminar_laptop para remover equipos que ya no requieran mantenimiento o seguimiento.**

# 📋 Resultados

Los datos se almacenan de forma descentralizada en la blockchain de Solana y pueden ser consultados en la sección de Accounts de tu programa. Un registro de inventario exitoso se visualiza de la siguiente manera:


```json
[
  {
    "publicKey": "BCt8hotZJ8k5e9TUhB2uJDqKMrxe1mCAsosui6g4aXXS",
    "account": {
      "owner": "9hmBa8YS5gGoC87oor3LUKvLXMmkBvFAcnKVu3Jn7VZ4",
      "inventario": [
        {
          "marca": "dell",
          "modelo": "latitude",
          "procesador": "corei9",
          "serie": "12th",
          "anio": 2022
        },
        {
          "marca": "msi",
          "modelo": "katana",
          "procesador": "ultra7",
          "serie": "gf",
          "anio": 2024
        },
        {
          "marca": "lenovo",
          "modelo": "legion",
          "procesador": "corei7",
          "serie": "lenovo",
          "anio": 2025
        }
      ]
    }
  }
]



