use std::io;
use std::io::Write;

pub fn read_integer(message: &str) -> i16 {
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

pub fn read_double(message: &str) -> f64 {
    let mut number = String::new();

    print!("{message}: ");
    io::stdout().flush()
        .expect("¡Error!");

    io::stdin().read_line(&mut number)
        .expect("¡Error!");

    let number: f64 = number.trim().parse()
        .expect("¡Introduzca un número válido!");

    number
}
