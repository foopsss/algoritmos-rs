use functions::read_input;
use functions::draw_line;

fn main() {
    let mut eleccion = read_input::<char>("¿Desea calcular el porcentaje de alumnos por carrera de varias facultades? [S/N]");
    println!("");

    let mut nro_facu = 0;
    let mut total_isi = 0;
    let mut total_iem = 0;
    let mut total_iq = 0;
    let mut total_general = 0;
    let mut total_facu;
    let mut porc_isi;
    let mut porc_iem;
    let mut porc_iq;

    while eleccion == 'S' {
        nro_facu = nro_facu + 1;

        let eg_isi = read_input::<i16>("Ingrese la cantidad de egresados de ISI");
        let eg_iem = read_input::<i16>("Ingrese la cantidad de egresados de IEM");
        let eg_iq = read_input::<i16>("Ingrese la cantidad de egresados de IQ");

        total_isi = total_isi + eg_isi;
        total_iem = total_iem + eg_iem;
        total_iq = total_iq + eg_iq;
        total_facu = eg_isi + eg_iem + eg_iq;
        total_general = total_general + total_facu;
        porc_isi = (eg_isi * 100) / total_facu;
        porc_iem = (eg_iem * 100) / total_facu;
        porc_iq = (eg_iq * 100) / total_facu;

        println!("");
        println!("Facultad: {nro_facu}");
        println!("Los egresados de ISI componen el {porc_isi}% de los egresados.");
        println!("Los egresados de IEM componen el {porc_iem}% de los egresados.");
        println!("Los egresados de IQ componen el {porc_iq}% de los egresados.");

        println!("");
        eleccion = read_input::<char>("¿Desea añadir otra facultad? [S/N]");

        draw_line(85);
    }

    println!("El total general de egresados de todas las facultades es de {total_general} estudiantes.");
    println!("El total de egresados de ISI es de {total_isi} estudiantes.");
    println!("El total de egresados de IEM es de {total_iem} estudiantes.");
    println!("El total de egresados de IQ es de {total_iq} estudiantes.");
}
