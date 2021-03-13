use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("O que tem{:?}", read_text());
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let result = write_file(filename);

    println!("In file {:?}", result);
    //Recreate the file and dump the processed contents to it
}

fn read_text() -> Result<String, io::Error> {
    let f = File::open("src/poem.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn write_file(filename: &String) -> std::io::Result<()> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut result = String::with_capacity(contents.len());
    let chars = contents.chars();

    println!("Char text:\n{:?}", chars);
    for (i, c) in contents.chars().enumerate() {
        if contents.chars().nth(i) == Some('\n')
            && contents.chars().nth(i + 1) != Some('\n')
            && contents.chars().nth(i - 1) != Some('\n')
        {
            result.push(';');
        } else {
            result.push(c);
        }
    }
    let mut f = File::create("foo.txt")?;
    f.write_all(result.as_bytes())?;

    f.sync_all()?;
    Ok(())
}
