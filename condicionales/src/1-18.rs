use functions::read_input;

fn main() {
    let dividendo = read_input::<i16>("Ingrese el valor del dividendo");
    let divisor = read_input::<i16>("Ingrese el valor del divisor");

    let mut resto = dividendo;
    let mut cociente = 0;

    while resto - divisor >= 0 {
        resto = resto - divisor;
        cociente = cociente + 1;
    }

    println!("");
    println!("El valor del dividendo es {dividendo}.");
    println!("El valor del divisor es {divisor}.");
    println!("El valor del cociente es {cociente}.");
    println!("El valor del resto es {resto}.");
}
