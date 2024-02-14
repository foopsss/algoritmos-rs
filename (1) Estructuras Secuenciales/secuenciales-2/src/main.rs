use functions;

fn main() {
    let a: i16 = functions::read_number("Introduzca un valor para el coeficiente A");
    let b: i16 = functions::read_number("Introduzca un valor para el coeficiente B");
    let c: i16 = functions::read_number("Introduzca un valor para el coeficiente C");
    
    let discriminante: i16 = b.pow(2) - 4 * a * c;
    
    println!("El resultado del discriminante es: {discriminante}");
}
