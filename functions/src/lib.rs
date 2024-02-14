use std::io;
use std::io::Write;

pub fn read_number(x: &str) -> i16 {
    // Esta función permite leer un valor introducido por el usuario.
    // También permite mostrar un mensaje personalizado.
    // Devuelve un entero con signo de 16 bits para emplearla en general.
    let mut value = String::new();

    print!("{x}: ");
    io::stdout().flush()
        .expect("¡Error!");

    io::stdin().read_line(&mut value)
        .expect("¡Error!");

    let value: i16 = value.trim().parse()
        .expect("¡Introduzca un número válido!");

    value
}
