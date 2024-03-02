use functions::read_integer;

fn main() {
    let x = read_integer("Introduzca un valor para X");
    let y = read_integer("Introduzca un valor para Y");
    let z = read_integer("Introduzca un valor para Z");

    let med_geo = (x * y * z) / 3;

    println!("");
    println!("Valor de X: {x}");
    println!("Valor de Y: {y}");
    println!("Valor de Z: {z}");
    println!("Valor de la media geométrica: {med_geo}");
}
