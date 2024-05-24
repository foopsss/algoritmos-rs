use functions::read_input;

fn main() {
    let eg_isi = read_input::<i16>("Ingrese la cantidad de egresados de ISI");
    let eg_iem = read_input::<i16>("Ingrese la cantidad de egresados de IEM");
    let eg_iq = read_input::<i16>("Ingrese la cantidad de egresados de IQ");

    let total = eg_isi + eg_iem + eg_iq;
    let porc_isi = (eg_isi * 100) / total;
    let porc_iem = (eg_iem * 100) / total;
    let porc_iq = (eg_iq * 100) / total;

    println!();
    println!("Los egresados de ISI componen el {porc_isi}% de los egresados.");
    println!("Los egresados de IEM componen el {porc_iem}% de los egresados.");
    println!("Los egresados de IQ componen el {porc_iq}% de los egresados.");
}
