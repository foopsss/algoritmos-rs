use std::io;
use std::io::Write;

pub fn read_value(x: &str) -> u16 {
    // Esta función permite leer un valor introducido por el usuario.
    // También permite mostrar un mensaje personalizado.
    // Devuelve un entero sin signo de 16 bits para emplearla en general.
    let mut value = String::new();

    print!("{x}: ");
    io::stdout().flush()
        .expect("¡Error!");

    io::stdin().read_line(&mut value)
        .expect("¡Error!");

    let value: u16 = value.trim().parse()
        .expect("¡Introduzca un número válido!");

    value
}
