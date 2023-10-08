use std::io;

fn texto_numero() -> i32 {
    loop {
        //println!("Ingrese un número: ");
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



fn crea_matriz_numeros() -> [[i32; 2];2]{
    let mut matriz: [[i32; 2];2] = [[0; 2];2];
    
    println!("Esta matriz sera cuadrada de 2x2");

    for i in 0..4{
        println!("Ingrese el número que desea insertar:");
        let numero = texto_numero();
        println!("Ingrese la fila donde desea insertar el número:");
        let fila = texto_numero() as usize;
        println!("Ingrese la columna donde desea insertar el número:");
        let columna = texto_numero() as usize;
        matriz[fila][columna] = numero;
    }
    return matriz;

}

fn multiplicar_matrices(matriz1:[[i32;2];2], matriz2:[[i32;2];2]) -> (){
    let mut result = [[0; 2]; 2];
    for i in 0..matriz1.len() {
        for j in 0..matriz2[0].len() {
            for k in 0..matriz1[0].len() {
                result[i][j] += matriz1[i][k] * matriz2[k][j];
            }
        }
    }
    println!("la multiplicacion es: {:?}",result);
}



fn main(){
    println!("MATRIZ 1\n");
    let matriz1 = crea_matriz_numeros();
    println!("MATRIZ 2\n");
    let matriz2 = crea_matriz_numeros();
    println!("magtriz1 {:?} y matriz 2 {:?} ",matriz1,matriz2);
    multiplicar_matrices(matriz1,matriz2);

    // println!("{:?}", matriz2);
    // let matriznueva2 = modificar_matriz(matriz2);


}
