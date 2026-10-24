use std::fs::File;
use std::io::{self, Read};

/*
fn show_sc(t: &[u8]){
    for i in 0..2{
        print!("[C{}]", i+1);
    }
    println!("");
    for i in 2..4{
        print!("[C{}]", i+1);
    }
    println!("");
}*/
fn main() -> Result<(), E>{
    //let t: Vec<u8> = vec![1, 2, 3];
    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // show_sc(&t);
    Ok(())
}
