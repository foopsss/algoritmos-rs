use functions::read_input;

fn main() {
    let viaje_euros = read_input::<f64>("Importe en euros necesario para viajar a Europa");
    let ahorros_dolares = read_input::<f64>("Cantidad ahorrada en dólares");
    let dolar_a_euro = read_input::<f64>("Equivalencia de dólar a euro");
    let euro_a_peso = read_input::<f64>("Equivalencia de euro a peso");

    let ahorros_euros = ahorros_dolares * dolar_a_euro;
    let diferencia_pesos;

    println!();
    if viaje_euros > ahorros_euros {
        diferencia_pesos = (viaje_euros - ahorros_euros) * euro_a_peso;
        println!("Para poder cubrir los costos del viaje se requieren de otros {diferencia_pesos} pesos.");
    } else if viaje_euros == ahorros_euros {
        println!("Felicidades, posee la cantidad justa para realizar el viaje.");
    } else {
        diferencia_pesos = (ahorros_euros - viaje_euros) * euro_a_peso;
        println!("Existe un excedente de ahorros de {diferencia_pesos} pesos.");
    }
}
