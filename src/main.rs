use std::io;

mod board;
mod game;

use game::Game;

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
