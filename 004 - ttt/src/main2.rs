use std::io::{self, Write};

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

fn show_t(board: &[u8]) {
    for row in board.chunks(3) {
        for &cell in row {
            let mark = match cell {
                2 => '.',
                1 => 'O',
                _ => 'X',
            };
            print!("{mark} ");
        }
        println!();
    }
}

fn prompt(board: &mut [u8], turn: u8) -> io::Result<usize> {
    loop {
        let mark = if turn == 0 { 'X' } else { 'O' };
        print!("{mark}: where do you want to put this? [1..9]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let Ok(choice) = input.trim().parse::<usize>() else {
            println!("Please enter a number from 1 to 9.");
            continue;
        };
        if !(1..=9).contains(&choice) {
            println!("Please enter a number from 1 to 9.");
            continue;
        }

        let index = choice - 1;
        if board[index] != 2 {
            println!("That square is already occupied.");
            continue;
        }
        board[index] = turn;
        return Ok(index);
    }
}

fn check_won(moves: &[usize]) -> bool {
    WINNING_LINES
        .iter()
        .any(|line| line.iter().all(|position| moves.contains(position)))
}

fn main() -> io::Result<()> {
    let mut board = vec![2; 9];
    let mut p1 = Vec::new();
    let mut p2 = Vec::new();
    let mut winner = None;

    for turn_number in 0..9 {
        let turn = (turn_number % 2) as u8;
        let choice = prompt(&mut board, turn)?;
        let moves = if turn == 0 { &mut p1 } else { &mut p2 };
        moves.push(choice);

        if turn_number > 3 && check_won(moves) {
            winner = Some(if turn == 0 { 'X' } else { 'O' });
        }

        show_t(&board);
        if winner.is_some() {
            break;
        }
    }

    println!("p1:\n {:?}\n of {} elements", p1, p1.len());
    println!("p2:\n {:?}\n of {} elements", p2, p2.len());
    match winner {
        Some(mark) => println!("player {mark} won!"),
        None => println!("The game ended in a draw."),
    }
    Ok(())
}
