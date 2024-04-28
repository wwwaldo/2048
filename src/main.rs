use std::io;
use twentyfortyeight::Grid;
fn main() {
    let mut grid = Grid::new();
    grid.step();

    loop {
        grid.print();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");


        if let Some(ch) = input.trim().chars().next() {
            match ch {
                'w' => grid.move_up(),
                'a' => grid.move_left(),
                's' => grid.move_down(),
                'd' => grid.move_right(),
                _ => println!("Invalid input"),
            }
        } else {
            println!("Invalid input");
        }

        grid.step();
    }
}