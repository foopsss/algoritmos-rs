use functions::read_integer;

fn main() {
    let anio_act = read_integer("Introduzca el año actual");
    let mes_act = read_integer("Introduzca el mes actual");
    let dia_act = read_integer("Introduzca el día actual");
    let anio_nac = read_integer("Introduzca el año de nacimiento");
    let mes_nac = read_integer("Introduzca el mes de nacimiento");
    let dia_nac = read_integer("Introduzca el día de nacimiento");

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
