mod board;
mod game;
mod io;

use game::Game;

fn main() {

    let mut game = Game::new();

    game.display();

    loop {

        println!("Player {}, choose your cell", game.player);

        let choice = io::get_choice();

        match choice {
            Ok(io::Choice::Command(command)) => {
                match command {
                    io::Command::Quit => break,
                    io::Command::Clear => {
                        game.reset_board();
                        game.display();
                        continue;
                    }
                }
            },
            Ok(io::Choice::Cell((row, col))) => {
                game.play_turn(row, col);
            },
            Err(message) => println!("Error: {}", message)
        }

        game.display();

    }
}
