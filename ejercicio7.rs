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

    loop{
        println!("Ingrese un numero entre 0 y 10");
        largo = texto_numero();
        if largo <= 10 {
            break
        }
    }

    for indice in 0..largo as usize{
        // println!("{indice}");
        let numero = texto_numero();
        arreglo[indice] = numero;
    }

    return arreglo

}

fn minmax_arreglo(arreglo:[i32;10])-> (){
    let mut min: i32 = arreglo[0];
    let mut max:i32 = arreglo[0];
    //recorre el arreglo y compara 
    for i in 0..arreglo.len(){
        if arreglo[i] <= min{
            min = arreglo[i];
        }
        if arreglo[i] >= max{
            max = arreglo[i];
        }
    }
    println!("El minimo es {min} y el maximo es {max}");    
}

fn main(){

    let arreglo = crea_arreglo_numeros();
    println!("{:?}", arreglo);
    minmax_arreglo(arreglo);
}
