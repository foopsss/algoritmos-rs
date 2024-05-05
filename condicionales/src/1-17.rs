use functions::read_input;

fn main() {
    let multiplicando = read_input::<i16>("Ingrese un valor para el multiplicando");
    let multiplicador = read_input::<i16>("Ingrese un valor para el multiplicador");

    let mut producto = 0;

    for _i in 1..=multiplicador {
        producto += multiplicando;
    }

    println!();
    println!("El valor del producto es {producto}.");
}
