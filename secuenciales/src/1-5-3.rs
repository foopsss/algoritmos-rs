use functions::read_input;

fn main() {
    const TASA_PC: f64 = 0.12;
    const TASA_IMP: f64 = 0.7;
    const TASA_IVA: f64 = 0.21;

    let costo_pc = read_input::<f64>("Costo para el vendedor de la PC");
    let costo_imp = read_input::<f64>("Costo para el vendedor de la impresora");

    let ganancia_pc = costo_pc * TASA_PC;
    let ganancia_imp = costo_imp * TASA_IMP;

    let sumatoria = costo_pc + costo_imp + ganancia_pc + ganancia_imp;
    let iva = sumatoria * TASA_IVA;
    let total = sumatoria + iva;

    println!("");
    println!("El precio total de las ventas es de {total} pesos.");
}
