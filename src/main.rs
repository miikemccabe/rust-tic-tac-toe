use std::fmt;
use std::io;
// use rand::Rng;

#[derive(Clone, Copy)]
enum Value {
    Naught,
    Cross,
    None
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Value::Naught => write!(f, "○"),
            Value::Cross => write!(f, "⨯"),
            Value::None => write!(f, " "),
        }
    }
}

fn print_row(values: &[Value; 3], row_label: String) {
    println!("|          |          |          |");
    println!("|     {}    |     {}    |     {}    |   {}", values[0], values[1], values[2], row_label);
    println!("|          |          |          |");
}

fn print_grid(grid: &[[Value; 3]; 3]) {
    println!("     1           2          3     ");
    println!("+----------+----------+----------+");
    print_row(&grid[0], String::from("A"));
    println!("+----------+----------+----------+");
    print_row(&grid[1], String::from("B"));
    println!("+----------+----------+----------+");
    print_row(&grid[2], String::from("C"));
    println!("+----------+----------+----------+");
}

fn main() {

    let mut grid: [[Value; 3]; 3] = [
        [Value::None, Value::None, Value::None],
        [Value::None, Value::None, Value::None],
        [Value::None, Value::None, Value::None],
    ];

    let mut player: Value = Value::Cross;

    print_grid(&grid);

    loop {

        println!("Player {}, choose your cell", player);

        let mut cell = String::new();

        io::stdin()
            .read_line(&mut cell)
            .expect("Failed to read line");

        let choice = cell.trim();

        // The user should enter a two digit coordinate for the cell like A2 or 3B. Splitting on "" produces an empty
        // string on either end of the array so we filter these about before collecting them as a Vector of &str.
        let mut choice = choice.split("").filter(|x| !x.is_empty()).collect::<Vec<&str>>();

        // Sort the array so that the user can enter the row or column in either order
        choice.sort_unstable();

        let column = choice[0].parse::<usize>().unwrap_or(0);

        let row =  match choice[1].to_lowercase().as_ref() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            _ => 0
        };

        let mut success = false;

        // Set the selected cell to the current player
        grid[row][column - 1] = match grid[row][column - 1] {
            Value::None => {
                success = true;
                player
            },
            _ => grid[row][column - 1]
        };

        // If all went well, swap the current player
        if success {
            player = match player {
                Value::Cross => Value::Naught,
                Value::Naught => Value::Cross,
                Value::None => Value::None,
            };
        }

        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");
        println!(" ");

        print_grid(&grid);

    }
}
