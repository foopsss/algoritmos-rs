use functions::read_double;

fn main() {
    let b_mayor: f64 = read_double("Ingrese la longitud de la base mayor");
    let b_menor: f64 = read_double("Ingrese la longitud de la base menor");
    let altura: f64 = read_double("Ingrese la altura");

    let superficie: f64 = ( (b_mayor + b_menor) * altura ) / 2.0;

    println!("");
    println!("La superficie del trapecio es de {superficie} metros cuadrados.");
    println!("La longitud de la base mayor es de {b_mayor} metros.");
    println!("La longitud de la base menor es {b_menor} metros.");
    println!("La altura es de {altura} metros.");
}
