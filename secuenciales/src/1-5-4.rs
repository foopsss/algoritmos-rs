use functions::read_double;

fn main() {
    let b_mayor: f64 = read_double("Ingrese la longitud de la base mayor");
    let b_menor: f64 = read_double("Ingrese la longitud de la base menor");
    let altura: f64 = read_double("Ingrese la altura");

    let superficie: f64 = ( (b_mayor + b_menor) * altura ) / 2.0;

    println!("");
    println!("Superficie del trapecio: {superficie} metros cuadrados.");
    println!("Longitud de la base mayor: {b_mayor} metros.");
    println!("Longitud de la base menor: {b_menor} metros.");
    println!("Altura: {altura} metros.");
}
