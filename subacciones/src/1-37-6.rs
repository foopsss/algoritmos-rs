use functions::read_input;

fn calcular_descuento(precio_producto: u16, unidades_producto: u16) -> f64 {
    const DESC_DOCENA: f64 = 0.4;
    const DESC_MAYOR_DOCENA: f64 = 0.10;

    match unidades_producto {
        0..=6 => 0.0,
        7..=12 => (precio_producto * unidades_producto) as f64 * DESC_DOCENA,
        13..=u16::MAX => (precio_producto * unidades_producto) as f64 * DESC_MAYOR_DOCENA,
    }
}

fn main() {
    let precio = read_input::<u16>("Introduzca el precio del producto a llevar");
    let unidades = read_input::<u16>("Introduzca las unidades a llevar");
    let monto_descuento = calcular_descuento(precio, unidades);

    println!();
    println!("Le corresponde un descuento de {monto_descuento} pesos.");
}
