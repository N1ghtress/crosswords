mod crossword;

use crossword::*;

fn main() {
    let my_cross = Crossword::new(5);

    println!("{}", my_cross);
}