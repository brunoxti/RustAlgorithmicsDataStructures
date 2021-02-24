use std::io;
use std::io::prelude::*;

//Some odd values in matrix  
fn main() {
    let mut a = [0; 5];

    let mut valor = String::new();

    for i in 0..5 {
        print!("Input the value {}º. value: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<i32>().unwrap();
        valor.clear();
    }

    println!();

    let size = a.len();
    let mut result = 0;
    for i in 0..size {
        if a[i] % 2 != 0 {
            result += a[i]
        }
    }
    println!("The some odd values list is [{:}] ", result);
}
