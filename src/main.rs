mod crossword;

use crossword::*;

fn main() {
    let mut my_cross = Crossword::new(5);
    my_cross.populate_words();

    println!("{}", my_cross);
}
