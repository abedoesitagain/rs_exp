use std::io::{self, Write, stdout};

struct Entry {
    name: String,
    word: String,
    code: u16,
}

fn genrand() -> String {
    let mut ch = String::new();
    for _ in 0..5 {
        let n: u8 = rand::random_range(b'A'..=b'Z');
        ch.push(n as char);
    }
    return ch;
}

fn gencode(ch: String) -> u16 {
    let mut s=0;
    for c in ch.chars(){
        println!("{}", c);
        s += c as u16;
    }
    return s;
}

fn checkhappy(mut n: u16) -> bool{
    loop {
        let mut s=0;
        for c in n.to_string().chars(){
            s += c.to_digit(10).unwrap();
        }
        if s.to_string().len() == 1{
            n = s as u16;
            break;
        }else{
            n = s  as u16;
        }
    }
    return n == 1;
}

fn main() {
    let mut T: Vec<Entry> = Vec::new();
    loop{
        let name = loop {
            let mut input = String::new();
            print!("Give me your name: ");
            let _ = stdout().flush();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.len() < 20 {
                break input;
            }
        };
        let rand = genrand();
        let entry = Entry {
            name,
            word: rand.clone(),
            code: gencode(rand),

        };
        T.push(entry);
        let mut inp = Default::default();
        print!("Continue? [Y/N]: ");
        let _ = stdout().flush();
        io::stdin().read_line(&mut inp).expect("Failed");
        if inp.to_uppercase().contains("N"){
            break;
        }
    };
    for i in 0..T.len(){
        println!("{} -> {}: {}", T[i].name.trim(), T[i].word, T[i].code);
        println!("{}", checkhappy(T[i].code));
    }
}
