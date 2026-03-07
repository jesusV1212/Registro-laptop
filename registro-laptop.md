## 🗂️ Arquitectura de Persistencia Descentralizada
Este sistema abandona las bases de datos convencionales para implementar un modelo de Cuentas Derivadas del Programa (PDAs). Esta arquitectura garantiza que cada nodo de información (el inventario de laptops) esté mapeado de forma biyectiva a la identidad criptográfica del administrador, como en el caso de los equipos Lenovo, Dell y MSI.

## PDA

La localización de los activos en la red no depende de un ID incremental, sino de una derivación criptográfica 
compuesta por:
- **Semilla Operativa:** El prefijo constante "registro", que define el espacio de nombres del contrato.
- **Firmante Autorizado:** La publicKey del owner, asegurando que solo el creador del registro tenga privilegios de
escritura sobre equipos.

## Administrar Espacio
- **Especificaciones tecnicas:** Cadenas de texto (String) con límites definidos para marca, modelo, procesador y serie, evitando el desperdicio de lamports en la red.

- **Inventario Dinamico** Un vector (Vec<Laptop>) con un máximo de 10 unidades, lo que permite almacenar múltiples equipos en una sola cuenta, optimizando el costo de almacenamiento en la blockchain de Solana..




 


