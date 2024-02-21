# algoritmos-rs
En este repositorio se encuentran ejercicios resueltos y escritos en [Rust](https://www.rust-lang.org/), el lenguaje de programación de sistemas de Mozilla, de la [guía de trabajos prácticos](https://aed-frre.github.io/) de la cátedra de Algoritmos y Estructuras de Datos de la UTN FRRe. Las versiones escritas en seudocódigo pueden encontrarse en otro de mis repositorios, [algoritmos](https://github.com/foopsss/algoritmos).

Sin embargo, a menos que se indique expresamente lo contrario, mis resoluciones son de la guía en formato PDF disponible en el campus virtual, y son probadas únicamente en sistemas operativos GNU/Linux, por lo que no puedo garantizar su correcto funcionamiento en otras plataformas.

## Diseño
El repositorio está dividido en varios proyectos de Cargo (o *crates* en inglés): uno para desarrollar una librería donde abstraigo algunas funciones comunes, y uno por cada tema de la guía, que contiene múltiples archivos de código fuente para cada ejercicio relacionado.

Los nombres de los proyectos de Cargo dan a entender el tema al que corresponden. Para consultar los nombres de cada ejercicio a la hora de compilar, recomiendo mirar el manifiesto (`Cargo.toml`) de cada proyecto, ya que ahí van a estar definidos los nombres de los ejecutables que van a producir.

* Siempre utilizo para los nombres el número correspondiente al enunciado en la guía, pero sustituyendo los puntos por guiones.

## Compilación
Para compilar los ejercicios hay que clonar el repositorio, situarse en el directorio/proyecto de Cargo correspondiente a un tema específico y utilizar el comando `cargo run --bin <número de ejercicio>`. Esto se va a encargar de compilar y ejecutar automáticamente el programa.

* Por ejemplo, si quisiera compilar y ejecutar el ejercicio 1.5.1 del Trabajo Práctico N.º 1 sobre Estructuras Secuenciales, me situaría en el proyecto de Cargo «secuenciales» y luego utilizaría el comando `cargo run --bin <1-5-1>`.

Si únicamente se busca compilar uno de los ejercicios, se puede utilizar el comando `cargo build --bin <número de ejercicio>`.
