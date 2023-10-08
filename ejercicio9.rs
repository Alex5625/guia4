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

fn bubble_sort(arreglo:[i32;10]) {
    let n = arreglo.len();
    let mut orden: [i32;10] = arreglo; 
    for indice in 0..n {
        for j in 0..n-indice-1 {
            // println!("POSICION PRINCIPAL {i} SECUNDARIA {j}\n");
            if orden[j] > orden[j+1] {
                //para asignar el cambio
                let temporal = orden[j];
                orden[j] = orden[j+1];
                orden[j+1] = temporal;
                // println!(" el arreglo va cambiando {:?}",orden);
            } else {

                // println!("No hubo cambio {:?}",orden);
            }
        }
    }
    println!("El arreglo ordenado es: {:?}",orden);
}


fn main(){

    let arreglo = crea_arreglo_numeros();
    println!("{:?}", arreglo);
    bubble_sort(arreglo);

}
