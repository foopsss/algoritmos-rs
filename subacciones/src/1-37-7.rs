use functions::read_input;

fn verificar_digito(entrada: char) {
    let numeros = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut coincide_numero = false;

    for numero in numeros {
        if entrada == numero {
            coincide_numero = true;
            break;
        }
    }

    if coincide_numero {
        println!("El carácter {entrada} es un dígito.");
    } else {
        println!("El carácter {entrada} NO es un dígito.");
    }
}

fn main() {
    let car_us = read_input::<char>("Introduzca un carácter");

    verificar_digito(car_us);
}
