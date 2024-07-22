use functions::{draw_line, read_input};

fn suma(num1: f64, num2: f64) {
    println!("El resultado de la suma es: {}", num1 + num2);
}

fn resta(num1: f64, num2: f64) {
    println!("El resultado de la resta es: {}", num1 - num2);
}

fn multiplicacion(num1: f64, num2: f64) {
    println!("El resultado de la multiplicación es: {}", num1 * num2);
}

fn division(num1: f64, num2: f64) {
    println!("El resultado de la división es: {}", num1 / num2);
}

fn chequear_valor(lim_inf: u8, lim_sup: u8) -> Result<u8, String> {
    // lim_inf corresponde a la mínima opción aceptable
    // y lim_sup a la máxima.
    let valor = read_input::<u8>("¿Qué operación desea realizar?");

    if valor >= lim_inf && valor <= lim_sup {
        Ok(valor)
    } else {
        Err("¡Introduzca un valor válido!".to_string())
    }
}

fn mostrar_menu() {
    println!("Operaciones");
    draw_line(15);
    println!("1. Sumar");
    println!("2. Restar");
    println!("3. Multiplicar");
    println!("4. Dividir");

    println!();
    match chequear_valor(1, 4) {
        Ok(operacion) => {
            let num_a = read_input::<f64>("Introduzca un número A");
            let num_b = read_input::<f64>("Introduzca un número B");

            println!();
            match operacion {
                1 => suma(num_a, num_b),
                2 => resta(num_a, num_b),
                3 => multiplicacion(num_a, num_b),
                4 => division(num_a, num_b),
                0 | 5..=u8::MAX => {}
            }
        }
        Err(err) => {
            println!();
            eprintln!("{err}");
        }
    }
}

fn main() {
    mostrar_menu();
}
