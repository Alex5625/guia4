//Ignacio Morales  Alexis Hernandez  Sofia Mieres

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

//funcion que elimina los ceros de la derecha del arreglo (si es q hay)
fn slice_array(arreglo: [i32;10])->(){
    //determina el largo del arreglo
    let mut contador: usize = 0;
    for i in 0..10 {
        let elemento : i32= arreglo[i];
        if elemento != 0 {
            contador = contador + 1;
        } 
    }
    //el arreglo se corta hasta el largo de "contador"
    let slice: &[i32] = &arreglo[0..contador];
    println!("El arreglo recortado es: {:?}", slice);

}
fn main(){

    let arreglo: [i32; 10] = crea_arreglo_numeros();
    println!("El arreglo original es: {:?}", arreglo);
    slice_array(arreglo);

}
