use functions::read_input;

fn main() {
    let numero = read_input::<i16>("Inserte un número que sea mayor que 100 y menor que 1000");

    if numero > 100 && numero < 1000 {
        let centenas = numero / 100;
        let decenas = (numero - centenas * 100) / 10;
        let unidades = numero - (centenas * 100) - (decenas * 10);

        println!("");
        println!("El número cuenta con {centenas} centenas.");
        println!("El número cuenta con {decenas} decenas.");
        println!("El número cuenta con {unidades} unidades.");

        if numero % 3 == 0 {
            println!("El número es múltiplo de 3.");
        } else {
            println!("El número NO es múltiplo de 3.");
        }
    } else {
        println!("El número no es mayor que 100 y menor que 1000.");
    }
}
