use core::fmt;
use std::io;
use std::io::Write;
use std::str;

pub fn read_input<T>(message: &str) -> T
where
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Debug,
{
    print!("{message}: ");

    io::stdout().flush().expect("¡Error al limpiar stdout!");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("¡Error al leer el valor!");

    let input: T = input.trim().parse().expect("¡Introduzca un valor válido!");

    input
}

pub fn draw_line(length: u8) {
    for _i in 1..(length - 1) {
        print!("-");
    }
    println!("-");
}
