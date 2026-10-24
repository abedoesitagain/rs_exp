use std::io;
use std::io::Write;

fn take() -> String {
    let mut buff = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buff).expect("a 100 dollas nigga");
    return buff
}
fn main() {
    print!("Give a phrase: ");
    io::stdout().flush().expect("a 100 dollas nigga");
    let userinput=take();
    print!("{}", userinput);
}
