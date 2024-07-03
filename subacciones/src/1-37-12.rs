use functions::read_input;

fn calcular_precio_final(precio_prod: f64, unidades_prod: u8) -> f64 {
    const DESC_DOCENA: f64 = 0.4;
    const DESC_MAYOR_DOCENA: f64 = 0.10;

    let total_sin_descuento = precio_prod * unidades_prod as f64;

    match unidades_prod {
        0..=6 => total_sin_descuento,
        7..=12 => total_sin_descuento - (total_sin_descuento * DESC_DOCENA),
        13..=u8::MAX => total_sin_descuento - (total_sin_descuento * DESC_MAYOR_DOCENA),
    }
}

fn main() {
    let precio = read_input::<f64>("Introduzca el precio del producto a llevar");
    let unidades = read_input::<u8>("Introduzca las unidades a llevar");
    let precio_final = calcular_precio_final(precio, unidades);

    println!();
    println!("El precio final a pagar es de {precio_final} pesos.");
}
