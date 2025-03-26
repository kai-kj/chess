
mod board;
mod ui;
use board::Board;
use ui::interface::setup;

fn main() {
    let b = Board::starting_position();
    // print!("{:?}", b.get_square(6, 6));
    setup(b);
}
