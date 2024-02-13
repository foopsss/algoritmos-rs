# algoritmos-rs
En este repositorio se encuentran ejercicios resueltos y escritos en [Rust](https://www.rust-lang.org/), el lenguaje de programación de sistemas de Mozilla, de la [guía de trabajos prácticos](https://aed-frre.github.io/) de la cátedra de Algoritmos y Estructuras de Datos de la UTN FRRe. Las versiones escritas en seudocódigo pueden encontrarse en otro de mis repositorios, [algoritmos](https://github.com/foopsss/algoritmos).

Sin embargo, a menos que se indique expresamente lo contrario, mis resoluciones son de la guía en formato PDF disponible en el campus virtual, y son probadas únicamente en sistemas operativos GNU/Linux, por lo que no puedo garantizar su correcto funcionamiento en otras plataformas.

## Diseño
El repositorio está dividido en varios proyectos de Cargo (o *crates* en inglés), uno para desarrollar una librería donde abstraigo algunas funciones comunes, y los otros donde tengo cada ejercicio.

## Compilación
Para compilar los ejercicios únicamente hay que clonar el repositorio, situarse en el directorio correspondiente a un proyecto de Cargo específico y utilizar el comando `cargo run`. Esto se va a encargar de compilar y ejecutar automáticamente el programa.

Si únicamente se busca compilar uno de los ejercicios, se puede utilizar el comando `cargo build`.