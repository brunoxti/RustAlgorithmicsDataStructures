use std::io;
use std::io::prelude::*;
//Soma 
fn main() {
    let mut a = [0; 10];
    let mut b = [0; 10];

    let mut valor = String::new();

    for i in 0..10 {
        print!("Entre com o {}º. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<i32>().unwrap();
        valor.clear();
    }
    println!();

    for i in 0..10 {
        if i % 2 == 0 {
            b[i] = a[i] * 5
        } else {
            b[i] = a[i] + 5
        }

        println!("A[{:2}] = {:4} na posição {}.", i + 1, a[i], i);

        println!("B[{}] = {} na posição {}.", i + 1, b[i], i);
    }
}
