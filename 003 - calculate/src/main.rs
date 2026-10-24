use std::io;
use std::io::Write;

fn getop(what: String, opts: String) -> String {
    print!("Give {} [{}]: ", what, opts);
    io::stdout().flush().expect("a 100 dollas nigga"); // flush stdin so print! content shows up
    let mut buff = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buff).expect("a 100 dollas nigga");
    return buff
}

fn getnum() -> i32 {
    print!("Give a signer 32 bit number: ");
    io::stdout().flush().expect("a 100 dollas nigga");
    let mut buff = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buff).expect("a 100 dollas nigga");
    return buff.trim_end().parse().expect("Not a valid int");
}

fn main(){
    let N1: i32 = getnum();
    const N: usize = 4;
    let t: [String; N] = ["+".to_string(), "-".to_string(), "*".to_string(), "/".to_string()];
    let mut opt = t[0].clone();
    for i in 1..N {
        opt = opt.to_string()+", "+&t[i];
    }
    let mut choice = String::new();
    let mut condition = false;
    while !(condition) {
        choice = getop("fag".to_string(), opt.clone());
        for i in 0..N {
            if choice.trim_end() == t[i] {
                println!("matched! {}", choice.trim_end());
                condition = true;
            }
        }
    }
    println!("{}", choice.trim_end());
    let N2: i32 = getnum();
    let rslt: i32;
    match choice.trim_end(){
        "+" => rslt = N1 + N2,
        "-" => rslt = N1 - N2,
        "*" => rslt = N1 * N2,
        "/" => rslt = N1 / N2,
        _  => rslt = -1
    }
    println!("{} {} {} = {}", N1, choice.trim_end(), N2, rslt);
}
