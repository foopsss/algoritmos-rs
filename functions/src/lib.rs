use std::io;
use std::io::Write;

pub fn read_number(message: &str) -> i16 {
    // Esta función permite leer un valor introducido por el usuario.
    // También permite mostrar un mensaje personalizado.
    // Devuelve un entero con signo de 16 bits para emplearla en general.
    let mut number = String::new();

    print!("{message}: ");
    io::stdout().flush()
        .expect("¡Error!");

    io::stdin().read_line(&mut number)
        .expect("¡Error!");

    let number: i16 = number.trim().parse()
        .expect("¡Introduzca un número válido!");

    number
}
