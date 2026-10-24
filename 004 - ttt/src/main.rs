use std::io::{self, Write};

fn show_t(t: &[u8]) {
    for i in 0..3 {
        for j in (i * 3)..(i * 3 + 3) {
            if t[j] == 2{
                print!(". ");
            }else if t[j] == 1{
                    print!("O ");
            }else{
                    print!("X ");
            }
        }
        print!("\n");
    }
}

fn prompt(t: &mut Vec<u8>, turn: u8) {
    loop {
        print!("{turn}: where do you want to put this? [1..9]: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Failed to read input");

        let mut index: usize = buff.trim().parse().expect("Not a valid integer");
        index -= 1;

        let cell = t.get_mut(index).expect("Index must be between 0 and 8");
        if *cell != 2 {
            println!("That spot is already picked. Try again.");
            continue;
        }

        *cell = turn;
        break;
    }
}

const WINNING_LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

fn check_won(table: &[u8], player: u8) -> bool {
    WINNING_LINES
        .iter()
        .any(|line| line.iter().all(|&position| table[position] == player))
}

fn main() {
    let mut table: Vec<u8> = vec![2; 9];
    for i in 0..9{
        let turn = i % 2;
        prompt(&mut table, turn);
        if check_won(&table, turn){
            println!("Player {turn} won!");
            show_t(&table);
            break;
        }
        show_t(&table);
    }
}
