use functions::read_input;

fn sumar_digitos(num: i64) -> i64 {
    let centenas = num / 100;
    let decenas = (num - centenas * 100) / 10;
	let unidades = num - (centenas * 100) - (decenas * 10);

    centenas + decenas + unidades
}

fn main() {
    let numero = read_input::<i64>("Introduzca un número de tres dígitos");

    let suma_digitos = sumar_digitos(numero);

    println!("");
    println!("La suma de los dígitos de {numero} es {suma_digitos}.");
}
