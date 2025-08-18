use chess::{ALL_SQUARES, BitBoard, Color, File, Piece, Rank, Square};
use iced::{
    Point, Rectangle, Size, Vector,
    advanced::image,
    widget::canvas::{self, Path, Stroke},
};

use crate::{
    opiece::{
        self,
        font::{COL, ROW},
    },
    style::chess_board::Style,
};

use super::{BState, overlay::Overlay};

pub struct Pieces {
    white: [image::Handle; 6],
    black: [image::Handle; 6],
}

impl Pieces {
    fn new(base_path: &str) -> Self {
        const PIECE_CHARS: [char; 6] = ['p', 'n', 'b', 'r', 'q', 'k'];

        Self {
            white: PIECE_CHARS.map(|c| format!("{base_path}/w{c}.png").into()),
            black: PIECE_CHARS.map(|c| format!("{base_path}/b{c}.png").into()),
        }
    }

    fn get(&self, piece: Piece, color: Color) -> &image::Handle {
        let set = match color {
            Color::White => &self.white[piece as usize],
            Color::Black => &self.black[piece as usize],
        };
        set
    }
}

pub struct ChessBoardRenderer {
    tile_size: f32,
    tile: Size,
    style: Style,
    state: BState,
    pieces: Pieces,
}

impl ChessBoardRenderer {
    pub fn new(style: Style, state: BState, bounds: Rectangle) -> Self {
        let pieces = Pieces::new("assets/pieces");
        let tile_size = bounds.width.min(bounds.height) / 8.0;
        let tile = Size::new(tile_size, tile_size);
        Self {
            tile_size,
            tile,
            style,
            state,
            pieces,
        }
    }

    fn piece_image(&self, square: Square) -> Option<&image::Handle> {
        let piece = self.state.game.board.piece_on(square)?;
        let color = self.state.game.board.color_on(square)?;
        Some(self.pieces.get(piece, color))
    }

    // fn piece_piece(&self, square: Square) -> Option<(Piece, Color)> {
    //     let piece = self.board.piece_on(square)?;
    //     let color = self.board.color_on(square)?;
    //     Some((piece, color))
    // }

    fn square_position(&self, square: &Square) -> Point {
        let row = square.get_rank().to_index();
        let col = square.get_file().to_index();
        self.tile_position(row, col)
    }

    fn tile_position(&self, row: usize, col: usize) -> Point {
        let row = if self.state.flipped { row } else { 7 - row };
        let col = if self.state.flipped { 7 - col } else { col };
        Point::new(col as f32 * self.tile_size, row as f32 * self.tile_size)
    }

    pub fn draw_board(&self, frame: &mut canvas::Frame, _overlay: &Overlay) {
        for row in 0..8 {
            for col in 0..8 {
                let pos = self.tile_position(row, col);

                let color = if (row + col) % 2 == self.state.flipped.into() {
                    self.style.board.dark
                } else {
                    self.style.board.light
                };

                frame.fill_rectangle(pos, self.tile, color);
            }
        }

        let colorow = if self.state.flipped { 7 } else { 0 };
        for row in 0..8 {
            let pos = self.tile_position(row, colorow)
                + Vector::new(self.tile_size / 15.0, self.tile_size / 10.0);

            let color = if (row + colorow) % 2 == self.state.flipped.into() {
                self.style.board.light
            } else {
                self.style.board.dark
            };

            opiece::font::draw(
                frame,
                &ROW[row],
                0.16 * self.tile_size,
                pos,
                color,
                false,
                false,
            );
        }

        for col in 0..8 {
            let pos = self.tile_position(colorow, col) + self.tile.into()
                - Vector::new(self.tile_size / 15.0, self.tile_size / 10.0);

            let color = if (col + colorow) % 2 == self.state.flipped.into() {
                self.style.board.light
            } else {
                self.style.board.dark
            };

            opiece::font::draw(
                frame,
                &COL[col],
                0.16 * self.tile_size,
                pos,
                color,
                true,
                true,
            );
        }
    }

    pub fn draw_board_overlay(&self, frame: &mut canvas::Frame, overlay: &Overlay) {
        if let Some(pos) = overlay.drag {
            let square = Square::make_square(
                Rank::from_index(pos.y as usize),
                File::from_index(pos.x as usize),
            );

            let width = self.tile_size * 0.05;
            let pos = self.square_position(&square) + Vector::new(width / 2.0, width / 2.0);
            let size = Size::new(self.tile_size - width, self.tile_size - width);
            frame.stroke(
                &Path::rectangle(pos, size),
                Stroke::default()
                    .with_width(width)
                    .with_color(self.style.overlay.hover),
            );
        }

        let mut highlights = BitBoard::new(0);
        for square in ALL_SQUARES {
            if BitBoard::from_square(square) & overlay.highlight != BitBoard::new(0) {
                highlights |= BitBoard::from_square(square);
                frame.fill_rectangle(
                    self.square_position(&square),
                    self.tile,
                    self.style.overlay.highlight,
                );
            }
        }

        if let Some(ant) = self.state.game.annotation {
            for sq in [ant.mv.get_source(), ant.mv.get_dest()] {
                if BitBoard::from_square(sq) & highlights == BitBoard::new(0) {
                    frame.fill_rectangle(
                        self.square_position(&sq),
                        self.tile,
                        self.style.overlay.prev_move,
                    );
                }
            }
        }

        if let Some(s_square) = overlay.selected {
            frame.fill_rectangle(
                self.square_position(&s_square),
                self.tile,
                self.style.overlay.selected,
            );
        }

        for mv in overlay.hints.iter() {
            let target = mv.get_dest();
            let pos = self.square_position(&target);
            let center = Point::new(
                pos.x + self.tile.width / 2.0,
                pos.y + self.tile.height / 2.0,
            );

            if self.state.game.board.piece_on(target).is_some() {
                let width = self.tile_size * 0.084;
                let radius = self.tile_size / 2.0 - width / 2.0;
                frame.stroke(
                    &Path::circle(center, radius),
                    Stroke::default()
                        .with_width(width)
                        .with_color(self.style.overlay.drag),
                );
            } else {
                let radius = self.tile_size * 0.168;
                let circle = Path::circle(center, radius);
                frame.fill(&circle, self.style.overlay.drag);
            }
        }
    }

    pub fn draw_pieces(&self, frame: &mut canvas::Frame, overlay: &Overlay) {
        for row in 0..8 {
            let rank = Rank::from_index(row);
            for col in 0..8 {
                let file = File::from_index(col);
                let square = Square::make_square(rank, file);

                if overlay.drag.is_some() && overlay.selected == Some(square) {
                    continue;
                }

                let pos = self.tile_position(row, col);

                // if let Some((piece, color)) = self.piece_piece(square) {
                //     if piece == Piece::Pawn && color == Color::Black {
                //         opiece::pawn::draw(frame, self.cell_size, pos);
                //         continue;
                //     }
                // }

                if let Some(img) = self.piece_image(square) {
                    frame.draw_image(
                        Rectangle::new(pos, self.tile),
                        image::Image::new(img.clone()),
                    );
                }
            }
        }
    }

    pub fn draw_drag(&self, frame: &mut canvas::Frame, overlay: &Overlay) {
        if let (Some(square), Some(pos)) = (overlay.selected, overlay.drag) {
            if let Some(img) = self.piece_image(square) {
                let row = if self.state.flipped {
                    pos.y
                } else {
                    8.0 - pos.y
                };
                let col = if self.state.flipped {
                    8.0 - pos.x
                } else {
                    pos.x
                };
                let pos = Point::new(col * self.tile_size, row * self.tile_size);
                let apos = pos - Vector::new(self.tile_size / 2.0, self.tile_size / 2.0);
                frame.draw_image(
                    Rectangle::new(apos, self.tile),
                    image::Image::new(img.clone()),
                );
            }
        }
    }

    pub fn draw_arrows(&self, frame: &mut canvas::Frame, overlay: &Overlay) {
        for mv in overlay.arrows.iter() {
            let spos = self.square_position(&mv.get_source());
            let dpos = self.square_position(&mv.get_dest());

            let dx = ((dpos.x - spos.x) / self.tile_size).round() as i32;
            let dy = ((dpos.y - spos.y) / self.tile_size).round() as i32;
            let angle = (dy as f32).atan2(dx as f32);

            const TAIL: &[(f32, f32)] = &[(0.36, 0.11), (0.36, -0.11)];
            const ARROW: &[(f32, f32)] = &[
                (-0.36, -0.11),
                (-0.36, -0.26),
                (-0.00, -0.00),
                (-0.36, 00.26),
                (-0.36, 00.11),
            ];

            let transform = |px: f32, py: f32, base: Point, angle: f32| -> Point {
                let x = px * self.tile_size;
                let y = py * self.tile_size;

                let rx = x * angle.cos() - y * angle.sin();
                let ry = x * angle.sin() + y * angle.cos();

                Point::new(
                    base.x + rx + self.tile_size / 2.0,
                    base.y + ry + self.tile_size / 2.0,
                )
            };

            let ddx = dx.abs();
            let ddy = dy.abs();

            let path = Path::new(|p| {
                if ddx.min(ddy) == 1 && ddx.max(ddy) == 2 {
                    let zy = if (dy * dx < 0) != (ddx > ddy) {
                        0.11
                    } else {
                        -0.11
                    };

                    let (angl0, angl1) = if ddx > ddy {
                        (0.0f32.atan2(dx as f32), (dy as f32).atan2(0.0))
                    } else {
                        ((dy as f32).atan2(0.0), 0.0f32.atan2(dx as f32))
                    };

                    for &(px, py) in TAIL {
                        p.line_to(transform(px, py, spos, angl0));
                    }
                    p.line_to(transform(2.00 + zy, -0.11, spos, angl0));
                    for &(px, py) in ARROW {
                        p.line_to(transform(px, py, dpos, angl1));
                    }
                    p.line_to(transform(2.00 - zy, 00.11, spos, angl0));
                } else {
                    for &(px, py) in TAIL {
                        p.line_to(transform(px, py, spos, angle));
                    }
                    for &(px, py) in ARROW {
                        p.line_to(transform(px, py, dpos, angle));
                    }
                }
                p.close();
            });

            frame.fill(&path, self.style.overlay.arrow);
        }
    }
}
