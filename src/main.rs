use std::fmt;
use std::io;
// use rand::Rng;

struct Board {
    grid: [[CellValue; 3]; 3]
}

impl Board {
    fn new() -> Self {
        Board {
            grid: [
                [CellValue::Empty, CellValue::Empty, CellValue::Empty],
                [CellValue::Empty, CellValue::Empty, CellValue::Empty],
                [CellValue::Empty, CellValue::Empty, CellValue::Empty],
            ]
        }
    }

    fn clear(&mut self) {
        self.grid = [
            [CellValue::Empty, CellValue::Empty, CellValue::Empty],
            [CellValue::Empty, CellValue::Empty, CellValue::Empty],
            [CellValue::Empty, CellValue::Empty, CellValue::Empty],
        ]
    }

    fn get_cell(&self, row: usize, col: usize) -> Option<&Player> {
        match &self.grid[row][col] {
            CellValue::Player(player) => Some(player),
            CellValue::Empty => None
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, value: Player) -> Result<(), ()> {

        let mut success = false;
        let cell = self.get_cell(row, col);

        self.grid[row][col] = match cell {
            None => {
                success = true;
                CellValue::Player(value)
            },
            Some(player) => CellValue::Player(*player)
        };

        match success {
            true => Ok(()),
            false => Err(())
        }
    }

    fn print_row(&self, row: usize) {
        let row_label = match row {
            0 => String::from("A"),
            1 => String::from("B"),
            2 => String::from("C"),
            _ => String::from("")
        };
        println!("|          |          |          |");
        println!("|     {}    |     {}    |     {}    |   {}", self.grid[row][0], self.grid[row][1], self.grid[row][2], row_label);
        println!("|          |          |          |");
    }

    fn display(&self) {
        println!("     1           2          3     ");
        println!("+----------+----------+----------+");
        self.print_row(0);
        println!("+----------+----------+----------+");
        self.print_row(1);
        println!("+----------+----------+----------+");
        self.print_row(2);
        println!("+----------+----------+----------+");
    }
}

enum CellValue {
    Player(Player),
    Empty
}

impl std::fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            CellValue::Player(player) => player.fmt(f),
            CellValue::Empty => write!(f, " ")
        }
    }
}

struct Game {
    board: Board,
    player: Player,
}

impl Game {

    fn new() -> Self {
        Game { 
            board: Board::new(),
            player: Player::Cross
        }
    }

    fn display(&self) {
        self.board.display();
    }

    fn reset_board(&mut self) {
        self.board.clear();
        self.player = Player::Cross;
    }

    fn toggle_player(&mut self) {
        self.player = match self.player {
            Player::Cross => Player::Naught,
            Player::Naught => Player::Cross
        }
    }

    fn play_turn(&mut self, row: usize, col: usize) {
        match self.board.set_cell(row, col, self.player) {
            Ok(()) => self.toggle_player(),
            Err(()) => ()
        }
    }
}

#[derive(Clone, Copy)]
enum Player {
    Naught,
    Cross
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Player::Naught => write!(f, "○"),
            Player::Cross => write!(f, "⨯")
        }
    }
}

fn main() {

    let mut game = Game::new();

    game.display();

    loop {

        println!("Player {}, choose your cell", game.player);

        let mut cell = String::new();

        io::stdin()
            .read_line(&mut cell)
            .expect("Failed to read line");

        let choice = cell.trim();

        if choice == "c" {
            game.reset_board();
            game.display();
            continue;
        } else if choice == "q" {
            println!("Thank you for playing!");
            break;
        }

        // The user should enter a two digit coordinate for the cell like A2 or 3B. Splitting on "" produces an empty
        // string on either end of the array so we filter these about before collecting them as a Vector of &str.
        let mut choice = choice.split("").filter(|x| !x.is_empty()).collect::<Vec<&str>>();

        // Sort the array so that the user can enter the row or column in either order
        choice.sort_unstable();

        let column = choice[0].parse::<usize>().unwrap_or(0) - 1;

        let row =  match choice[1].to_lowercase().as_ref() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            _ => 0
        };

        game.play_turn(row, column);

        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");

        game.display();

    }
}
