use std::io::{stdin, stdout, Write};
use std::{env, process};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Symbol {
    X,
    O,
}

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub symbol: Symbol,
}

impl Player {
    pub fn new(name: String, symbol: Symbol) -> Player {
        Player { name, symbol }
    }
}

#[derive(Clone)]
struct Players {
    player_one: Player,
    player_two: Player,
}

impl Players {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Players, &'static str> {
        args.next();

        let player_one_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get first name!"),
        };

        let player_two_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get second name!"),
        };

        let player_one: Player = Player::new(player_one_name, Symbol::X);
        let player_two: Player = Player::new(player_two_name, Symbol::O);
        Ok(Players {
            player_one,
            player_two,
        })
    }
    pub fn next_player(&self, current_player: Player) -> Player {
        if self.player_one.name == current_player.name {
            return self.player_two.clone();
        } else {
            return self.player_one.clone();
        }
    }
}

struct Board {
    board: [Option<Symbol>; 9],
}

impl Board {
    pub fn new() -> Board {
        Board { board: [None; 9] }
    }
    pub fn replace(&mut self, pos: usize, symbol: Symbol) {
        self.board[pos] = Some(symbol);
    }
    pub fn print_board(&self) {
        let board = self.board;
        fn show(s: &[Option<Symbol>; 9], n: usize) -> &'static str {
            match s[n] {
                Some(Symbol::X) => "X",
                Some(Symbol::O) => "O",
                None => match n {
                    0 => "0",
                    1 => "1",
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    5 => "5",
                    6 => "6",
                    7 => "7",
                    8 => "8",
                    9 => "9",
                    _ => unreachable!(),
                },
            }
        }
        println!(
            "{}",
            format!(
                "
    +---+---+---+
    | {} | {} | {} |
    +---+---+---+
    | {} | {} | {} |
    +---+---+---+
    | {} | {} | {} |
    +---+---+---+
    ",
                show(&board, 0),
                show(&board, 1),
                show(&board, 2),
                show(&board, 3),
                show(&board, 4),
                show(&board, 5),
                show(&board, 6),
                show(&board, 7),
                show(&board, 8),
            )
        );
    }
    pub fn has_winner(&self) -> bool {
        let board: [Option<Symbol>; 9] = self.board;
        let wins: [[usize; 3]; 8] = [
            [0, 1, 2],
            [0, 3, 6],
            [0, 4, 8],
            [1, 4, 7],
            [2, 5, 8],
            [2, 4, 6],
            [3, 4, 5],
            [6, 7, 8],
        ];
        wins.iter().any(|i| {
            board[i[0]].is_some() && board[i[0]] == board[i[1]] && board[i[0]] == board[i[2]]
        })
    }
}

pub fn capture_input(board: [Option<Symbol>; 9]) -> usize {
    let mut input = String::new();
    loop {
        print!("Position: ");
        let _ = stdout().flush();

        input.clear();
        stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");

        match input.trim().parse::<usize>() {
            Ok(i) if i > 8 => eprintln!("Input '{i}' is out of range"),
            Ok(i) if board[i].is_some() => {
                eprintln!("Position {i} is already taken with {:?}", board[i].unwrap())
            }
            Ok(i) => return i,
            Err(err) => eprintln!("Couldnt parse input: {err}"),
        };
    }
}

enum EndGame {
    Win,
    Draw,
}

fn main() {
    let mut board = Board::new();
    let players = Players::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let mut end_game: EndGame = EndGame::Draw;
    let mut current_player: Player = players.player_two.clone();
    for _ in 1..=9 {
        board.print_board();
        current_player = Players::next_player(&players, current_player);
        let input = capture_input(board.board.clone());
        board.replace(input, current_player.symbol);
        if board.has_winner() {
            end_game = EndGame::Win;
            break;
        }
    }
    board.print_board();
    match end_game {
        EndGame::Draw => println!("Draw!"),
        EndGame::Win => println!("{} won!", current_player.name),
    }
}
