use functions::read_input;

fn verificar_digito(entrada: char) -> Option<bool> {
    let numeros = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for numero in numeros {
        if entrada == numero {
            return Some(true);
        }
    }

    /* Por defecto, no se devuelve ningún valor.
    Esto se debe a que el tipo de dato que está
    anotado como retorno es Option<bool>, lo que
    significa que no siempre puede ser ese valor
    por más que yo asuma que siempre debería de
    resultar algo de tipo booleano. */
    None
}

fn main() {
    let car_us = read_input::<char>("Introduzca un carácter");
    let es_dígito = verificar_digito(car_us);

    println!();
    if es_dígito.expect("¡Se esperaba un valor booleano!") {
        println!("El carácter {car_us} es un dígito.");
    } else {
        println!("El carácter {car_us} NO es un dígito.");
    }
}

