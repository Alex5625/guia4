
use std::io;

fn texto_numero() -> i32 {
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

fn crea_arreglo_numeros() -> [i32; 10]{
    let mut arreglo: [i32; 10] = [0; 10];
    let mut largo = 0;

    //largo de los elementos a cambiar
    loop{
        println!("Ingrese un numero entre 0 y 10");
        largo = texto_numero();
        if largo <= 10 {
            break
        }
    }
    //indica los numeros para modificar los elementos del array
    for indice in 0..largo as usize{
        // println!("{indice}");
        let numero = texto_numero();
        arreglo[indice] = numero;
    }
    return arreglo;
}
//se deja como procedimiento
fn buscar_numero(arreglo: [i32;10])->(){
    let mut booleano:bool = true;
    println!("Dime un numero que quieras encontrar dentro del arreglo");
    let numero = texto_numero();

    for i in 0..arreglo.len(){
        if arreglo[i] == numero{
            booleano = true;
        } else {
            booleano = false;
        }
    }

    if booleano {
        println!("El numero {numero} se encuentra dentro del arreglo");
    }else{
        println!("El numero {numero} no se encuentra dentro del arreglo");
    }

}   

fn main(){

    let arreglo: [i32; 10] = crea_arreglo_numeros();
    println!("El arreglo original es: {:?}", arreglo);
    buscar_numero(arreglo);

}
