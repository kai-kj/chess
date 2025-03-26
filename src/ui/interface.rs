use std::{vec};
use fltk::{app, draw, enums::{Color}, image::{PngImage}, prelude::*, widget::Widget, window::Window};
use crate::board::{Board};

#[derive(Clone)]
struct ChessGrid {
    widget: Widget,
    board: Board,
}

impl ChessGrid{
    pub fn new(x: i32, y: i32, w: i32, h: i32, board: Board) -> Self {
        let widget = Widget::new(x, y, w, h, "");
        let mut grid = Self { 
            widget, 
            board,
        };

        grid.widget.draw(move |w| {
            let grid_size = 8;
            let square_width = w.width() / grid_size;
            let square_height = w.height() / grid_size;

            let positions = (0..grid_size).flat_map(move |row| {
                (0..grid_size).map(move |col| (row, col))
            });

            for (row, col) in positions {
                let color = match (row + col) % 2 == 0 {
                    true => Color::White,
                    false => Color::DarkGreen,
                };

                let x = w.x() + col * square_width;
                let y = w.y() + row * square_height;
                draw::draw_rect_fill(x, y, square_width, square_height, color);
                draw::draw_rect(x, y, square_width, square_height); 
            }

            let positions = vec![
                (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
                (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7),
                (7, 0), (7, 1), (7, 2), (7, 3), (7, 4), (7, 5), (7, 6), (7, 7),
                (6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5), (6, 6), (6, 7)
            ];

            for (row, col) in positions {
                let piece = board.get_square(row, col).unwrap().to_string();
                let path = load_piece(&piece);

                match PngImage::load(path) {
                    Ok(mut img) => {
                        img.scale(square_width as i32, square_height as i32, true, true);

                        let x = w.x() + col as i32 * square_width + (square_width - img.width() as i32) / 2;
                        let y = w.y() + row as i32 * square_height + (square_height - img.height() as i32) / 2;
            
                        img.draw(x, y, square_width, square_height);
                    }
                    Err(err) => eprintln!("Failed to load image {}: {}", path, err),
                }
            }
        });

        grid
    }
}

pub fn load_piece(piece: &str) -> &str {
    match piece {
        "b" => "src/images/b_bishop.png",
        "B" => "src/images/w_bishop.png",
        "n" => "src/images/b_knight.png",
        "N" => "src/images/w_knight.png",
        "p" => "src/images/b_pawn.png",
        "P" => "src/images/w_pawn.png",
        "r" => "src/images/b_rook.png",
        "R" => "src/images/w_rook.png",
        "k" => "src/images/b_king.png",
        "K" => "src/images/w_king.png",
        "q" => "src/images/b_queen.png",
        "Q" => "src/images/w_queen.png",
        _ => todo!(),
    }
}

pub fn setup(board: Board) {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 500, 500, "Chess");

    let _chess_grid = ChessGrid::new(50, 50, 400, 400, board);

    window.end();
    window.show();
    app.run().unwrap();
}
