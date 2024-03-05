use functions::read_integer;

fn main() {
    let multiplicando = read_integer("Ingrese un valor para el multiplicando");
    let multiplicador = read_integer("Ingrese un valor para el multiplicador");

    let mut producto = 0;

    for _i in 1..=multiplicador {
        producto = producto + multiplicando;
    }

    println!("");
    println!("El valor del producto es {producto}.");
}
