use std::io;
use std::io::prelude::*;

//use std::io::prelude::*;
fn main() {
    let a: [[i32; 4]; 4] = [[2,5,6,4],
                            [5,2,1,4],
                            [7,3,2,1],
                            [5,2,2,2]];

    let b: [[i32; 4]; 4] = [[9,6,6,5],
                            [4,5,1,2],
                            [5,1,2,1],
                            [4,4,4,2]];

    let mut result: [[i32; 4]; 4] = [[0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            result[i][j] = a[i][j] + b[i][j];
            print!("{:?} ", result[i][j]);
        }
        println!();
    }
}
