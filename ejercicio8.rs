use std::io;
use std::convert::TryInto;

fn texto_numero() -> i32 {
    println!("Longitud del arreglo Serie fibonacci");
    loop {
        println!("Ingrese un número: ");
        let mut numero = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut numero).unwrap();
        let numero: i32 = match numero.trim().parse(){
            Ok(numero) => numero,
            Err(_) => {
                println!("Error, no es un número");
                continue;
            },
        };
        return numero;
    }
}

fn fibonacci(n: usize) -> (){
    let mut arreglo_fibonacci: [u32; 100] = [0; 100];

    arreglo_fibonacci[0] = 0;
    arreglo_fibonacci[1] = 1;
    arreglo_fibonacci[2] = 1;

    for i in 3..n {
        arreglo_fibonacci[i] = arreglo_fibonacci[i - 1] + arreglo_fibonacci[i - 2];
    }
    //hace un slice al arreglo de tamaño 100 
    let arreglo_fibonacci: &[u32] = &arreglo_fibonacci[0..n];
    println!("la serie fibonacci es: {:?}",arreglo_fibonacci);
}

fn main() {
    let n = texto_numero();
    let largo:usize = n.try_into().unwrap();
    fibonacci(largo);
}



