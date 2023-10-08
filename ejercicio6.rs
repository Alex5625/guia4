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



fn crea_arreglo1_numeros() -> [i32; 10]{
    let mut arreglo: [i32; 10] = [0; 10];
    let mut largo = 0;

    loop{
        println!("Ingrese un numero entre 0 y 10 para el arreglo 1");
        largo = texto_numero();
        if largo <= 10 {
            break
        }
    }

    for indice in 0..largo as usize{
        //println!("{indice}");
        let numero = texto_numero();
        arreglo[indice] = numero;
    }

    return arreglo

}
fn crea_arreglo2_numeros() -> [i32; 10]{
    let mut arreglo: [i32; 10] = [0; 10];
    let mut largo = 0;

    loop{
        println!("Ingrese un numero entre 0 y 10 para el arreglo 2");
        largo = texto_numero();
        if largo <= 10 {
            break
        }
    }

    for indice in 0..largo as usize{
        //println!("{indice}");
        let numero = texto_numero();
        arreglo[indice] = numero;
    }

    return arreglo

}

fn multiplicacion_arreglos(arreglo1:[i32;10],arreglo2:[i32;10]) -> (){
    //let mut multiplicacion:[i32;10] = [0;10];
    println!("{:?}",multiplicacion);
    for i in 0..arreglo1.len(){
        multiplicacion[i] = arreglo1[i] * arreglo2[i];

    }
    println!("La multiplicacion de ambos arreglos es: {:?}",multiplicacion);
}


fn main(){

    let arreglo1 = crea_arreglo1_numeros();
    let arreglo2 = crea_arreglo2_numeros();
    //println!("{:?},{:?}",arreglo1,arreglo2);
    multiplicacion_arreglos(arreglo1,arreglo2);
}
