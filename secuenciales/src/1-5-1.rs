use functions::read_input;

fn main() {
    const TASA_INF: f64 = 0.04;
    
    let año_act = read_input::<f64>("Introduzca el año actual");
    let precio_act = read_input::<f64>("Introduzca el precio actual del artículo");
    let año_fut = read_input::<f64>("Introduzca un año a futuro");
    
    let precio_fut = precio_act * (1 as f64 + TASA_INF).powf(año_fut - año_act);

    println!("");
    println!("Para el año {año_fut} el artículo tendrá un precio de {precio_fut} pesos.");
}
