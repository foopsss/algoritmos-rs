use std::cmp::Ordering;
use functions::read_input;

fn main() {
    let anio_act = read_input::<i16>("Introduzca el año actual");
    let mes_act = read_input::<i16>("Introduzca el mes actual");
    let dia_act = read_input::<i16>("Introduzca el día actual");
    let anio_nac = read_input::<i16>("Introduzca el año de nacimiento");
    let mes_nac = read_input::<i16>("Introduzca el mes de nacimiento");
    let dia_nac = read_input::<i16>("Introduzca el día de nacimiento");

    let mut edad = anio_act - anio_nac;

    match mes_act.cmp(&mes_nac) {
        Ordering::Less => edad = (anio_act - anio_nac) - 1,
        Ordering::Equal => {
            if dia_act < dia_nac {
                edad = (anio_act - anio_nac) - 1;
            }
        }
        Ordering::Greater => {},
    }

    println!();
    println!("La persona tiene {edad} años.");
}
