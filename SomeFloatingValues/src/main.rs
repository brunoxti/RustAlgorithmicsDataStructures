use std::io;
use std::io::prelude::*;

//use std::io::prelude::*;
fn main() {
    let mut a = [0.; 5];

    let mut valor = String::new();

    for i in 0..5 {
        print!("Input value {}º. value: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<f64>().unwrap();
        valor.clear();
    }

    println!();

    let size = a.len();
    let mut result = 0.;

    for i in 0..size {
        result += a[i]
    }

    println!("The some float values is [{:2}] ", result);
}
