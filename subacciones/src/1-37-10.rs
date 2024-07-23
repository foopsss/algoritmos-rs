use functions::read_input;

fn mostrar_nro(entrada: u8) {
    match entrada {
        0 => {
            println!("****");
            println!("*  *");
            println!("*  *");
            println!("*  *");
            println!("****");
        }
        1 => {
            println!("   *");
            println!("   *");
            println!("   *");
            println!("   *");
            println!("   *");
        }
        2 => {
            println!("****");
            println!("   *");
            println!("****");
            println!("*   ");
            println!("****");
        }
        3 => {
            println!("****");
            println!("   *");
            println!("****");
            println!("   *");
            println!("****");
        }
        4 => {
            println!("*  *");
            println!("*  *");
            println!("****");
            println!("   *");
            println!("   *");
        }
        5 => {
            println!("****");
            println!("*   ");
            println!("****");
            println!("   *");
            println!("****");
        }
        6 => {
            println!("****");
            println!("*   ");
            println!("****");
            println!("*  *");
            println!("****");
        }
        7 => {
            println!("****");
            println!("   *");
            println!("   *");
            println!("   *");
            println!("   *");
        }
        8 => {
            println!("****");
            println!("*  *");
            println!("****");
            println!("*  *");
            println!("****");
        }
        9 => {
            println!("****");
            println!("*  *");
            println!("****");
            println!("   *");
            println!("****");
        }
        _ => println!("¡Introduzca un número correcto!"),
    }
}

fn main() {
    let numero = read_input::<u8>("Introduzca un número del 0 al 9");
    println!();
    mostrar_nro(numero);
}
