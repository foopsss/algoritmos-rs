use functions::read_integer;

fn main() {
    let dividendo = read_integer("Ingrese el valor del dividendo");
    let divisor = read_integer("Ingrese el valor del divisor");

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
