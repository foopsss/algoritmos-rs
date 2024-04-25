use functions::read_input;

fn main() {
    let n1_c1 = read_input::<i64>("Introduzca la primera coordenada del primer número complejo");
    let n1_c2 = read_input::<i64>("Introduzca la segunda coordenada del primer número complejo");
    let n2_c1 = read_input::<i64>("Introduzca la primera coordenada del segundo número complejo");
    let n2_c2 = read_input::<i64>("Introduzca la segunda coordenada del segundo número complejo");

    let prod_c1 = (n1_c1 * n2_c1) - (n1_c2 * n2_c2);
    let prod_c2 = (n1_c1 * n2_c2) + (n1_c2 * n2_c1);

    println!("");
    println!("El resultado del producto es: ({prod_c1} ; {prod_c2})");
}
