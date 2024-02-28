use functions::read_double;

fn main() {
    let costo_pc: f64 = read_double("Costo para el vendedor de la PC");
    let costo_imp: f64 = read_double("Costo para el vendedor de la impresora");

    let ganancia_pc: f64 = costo_pc * 0.12;
    let ganancia_imp: f64 = costo_imp * 0.7;

    let sumatoria: f64 = costo_pc + costo_imp + ganancia_pc + ganancia_imp;
    let iva: f64 = sumatoria * 0.21;
    let total: f64 = sumatoria + iva;

    println!("El precio total de las ventas es de {total} pesos.");
}
