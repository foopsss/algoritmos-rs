use funciones;

fn main() {
    const TASA_INF: f32 = 0.04;
    
    let año_act: u16 = funciones::read_value("Introduzca el año actual");
    let precio_act: u16 = funciones::read_value("Introduzca el precio actual del artículo");
    let año_fut: u16 = funciones::read_value("Introduzca un año a futuro");
    
    let precio_fut: f32 = precio_act as f32 * (1 as f32 + TASA_INF).powf(año_fut as f32 - año_act as f32);
    
    println!("Para el año {año_fut} el artículo tendrá un precio de {precio_fut} pesos.");
}
