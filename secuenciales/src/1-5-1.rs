use functions::read_double;

fn main() {
    const TASA_INF: f64 = 0.04;
    
    let año_act: f64 = read_double("Introduzca el año actual");
    let precio_act: f64 = read_double("Introduzca el precio actual del artículo");
    let año_fut: f64 = read_double("Introduzca un año a futuro");
    
    let precio_fut: f64 = precio_act * (1 as f64 + TASA_INF).powf(año_fut - año_act);

    println!("");
    println!("Para el año {año_fut} el artículo tendrá un precio de {precio_fut} pesos.");
}
