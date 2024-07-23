use functions::read_input;

fn es_primo(entrada: i32) -> bool {
    for i in 2..entrada {
        if entrada % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let numero = read_input::<i32>("Introduzca un número entre -2147483648 y 2147483647");

    println!();
    match numero {
        0 | 1 => println!("El número no es primo ni compuesto."),
        2 => println!("El número es primo."),
        i32::MIN..=-1 | 3..=i32::MAX => {
            if es_primo(numero) {
                println!("El número es primo.");
            } else {
                println!("El número es compuesto.");
            }
        }
    }
}
