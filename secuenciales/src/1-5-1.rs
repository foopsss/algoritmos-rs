use functions::read_integer;

fn main() {
    const TASA_INF: f64 = 0.04;
    
    let año_act: i16 = read_integer("Introduzca el año actual");
    let precio_act: i16 = read_integer("Introduzca el precio actual del artículo");
    let año_fut: i16 = read_integer("Introduzca un año a futuro");
    
    let precio_fut: f64 = precio_act as f64 * (1 as f64 + TASA_INF).powf(año_fut as f64 - año_act as f64);
    
    println!("Para el año {año_fut} el artículo tendrá un precio de {precio_fut} pesos.");
}
