use functions::read_input;

fn main() {
    let a = read_input::<i64>("Ingrese un valor para a");
    let b = read_input::<i64>("Ingrese un valor para b");
    let suma = a + b;

    println!("");
    match suma {
        i64::MIN ..= 50 => println!("La suma es menor o igual a 50"),
        51 ..= 100 => println!("La suma es mayor que 50 pero menor o igual a 100."),
        101 ..= 200 => println!("La suma es mayor que 100 pero menor o igual a 200."),
        201.. => println!("La suma es mayor que 200."),
    };
}
