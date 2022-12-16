mod crossword;

use crossword::*;

fn main() {
    let mut my_cross = Crossword::new(7, 50);
    my_cross.populate_words();
    my_cross.start();
}