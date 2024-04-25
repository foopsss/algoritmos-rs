use functions::read_input;

fn main() {
    let a = read_input::<i64>("Introduzca un valor para el coeficiente A");
    let b = read_input::<i64>("Introduzca un valor para el coeficiente B");
    let c = read_input::<i64>("Introduzca un valor para el coeficiente C");
    
    let discriminante = b.pow(2) - 4 * a * c;

    println!("");
    println!("El resultado del discriminante es: {discriminante}");
}
