use std::fmt;
// use std::io;
// use rand::Rng;

struct Row<'a> {
    position: RowPosition,
    values: &'a [Value; 3],
}

enum Value {
    Naught,
    Cross,
    None
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Value::Naught => write!(f, "O"),
            Value::Cross => write!(f, "X"),
            Value::None => write!(f, " "),
        }
    }
}

enum RowPosition {
    Top,
    Middle,
    Bottom,
}

fn print_row_border() {
    println!("+----------+----------+----------+");
}

fn print_row_middle(values: &[Value; 3]) {
    println!("|          |          |          |");
    println!("|     {}    |     {}    |     {}    |", values[0], values[1], values[2]);
    println!("|          |          |          |");
}

fn print_row(row: Row) {
    match row.position {
        RowPosition::Top => { 
            print_row_border();
            print_row_middle(row.values);
        },
        RowPosition::Middle => {
            print_row_border();
            print_row_middle(row.values);
            print_row_border();
        },
        RowPosition::Bottom => {
            print_row_middle(row.values);
            print_row_border();
        }
    }
}

fn print_grid(grid: [[Value; 3]; 3]) {
    let top_row = Row {
        position: RowPosition::Top,
        values: &grid[0],
    };
    let middle_row = Row {
        position: RowPosition::Middle,
        values: &grid[1],
    };
    let bottom_row = Row {
        position: RowPosition::Bottom,
        values: &grid[2],
    };
    print_row(top_row);
    print_row(middle_row);
    print_row(bottom_row);
}

fn main() {
    let grid: [[Value; 3]; 3] = [
        [Value::None, Value::Cross, Value::None],
        [Value::None, Value::None, Value::Naught],
        [Value::None, Value::Cross, Value::None],
    ];
    print_grid(grid);
}
