use functions::read_integer;

fn main() {
    let dinero = read_integer("Inserte una suma de dinero que sea mayor que 100 y menor que 1000");

    println!("");
    if dinero > 100 && dinero < 1000 {
        let centenas = dinero / 100;
        let decenas = (dinero - centenas * 100) / 10;
        let unidades = dinero - (centenas * 100) - (decenas * 10);

        println!("Se necesitan {centenas} billetes de 100 pesos.");
        println!("Se necesitan {decenas} billetes de 10 pesos.");
        println!("Se necesitan {unidades} billetes de 1 peso.");
    } else {
        println!("La suma de dinero no es mayor que 100 y menor que 1000.");
    }
}
