use functions::read_input;

fn main() {
    let anio_act = read_input::<i64>("Introduzca el año actual");
    let anio_nac = read_input::<i64>("Introduzca el año de nacimiento");
    let mes_act = read_input::<i64>("Introduzca el mes actual");
    let mes_nac = read_input::<i64>("Introduzca el mes de nacimiento");
    let dia_act = read_input::<i64>("Introduzca el día actual");
    let dia_nac = read_input::<i64>("Introduzca el día de nacimiento");

    let mut edad = anio_act - anio_nac;

    if mes_act == mes_nac {
        if dia_act < dia_nac {
            edad = (anio_act - anio_nac) - 1;
        }
    } else if mes_act < mes_nac {
        edad = (anio_act - anio_nac) - 1;
    }

    println!("");
    println!("La persona tiene {edad} años.");
}
