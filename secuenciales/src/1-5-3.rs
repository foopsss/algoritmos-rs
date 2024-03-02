use functions::read_double;

fn main() {
    let costo_pc = read_double("Costo para el vendedor de la PC");
    let costo_imp = read_double("Costo para el vendedor de la impresora");

    let ganancia_pc = costo_pc * 0.12;
    let ganancia_imp = costo_imp * 0.7;

    let sumatoria = costo_pc + costo_imp + ganancia_pc + ganancia_imp;
    let iva = sumatoria * 0.21;
    let total = sumatoria + iva;

    println!("");
    println!("El precio total de las ventas es de {total} pesos.");
}
