use std::io;
use std::io::Write;
use std::str;
use core::fmt;

pub fn read_input<T>(message: &str) -> T
where
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Debug,
{
    print!("{message}: ");

    io::stdout()
        .flush()
        .expect("¡Error al limpiar stdout!");

    let mut number = String::new();
    
    io::stdin()
        .read_line(&mut number)
        .expect("¡Error al leer el valor!");

    let number: T = number
        .trim()
        .parse()
        .expect("¡Introduzca un número válido!");

    number
}
