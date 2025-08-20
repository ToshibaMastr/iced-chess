use chess::{BitBoard, BoardStatus, ChessMove, File, MoveGen, Piece, Rank, Square};
use iced::{Point, Rectangle, advanced::Shell, mouse, widget::canvas};

use super::{BState, Caches, Messages};
use crate::{
    chess::Move,
    sound::{ChessBoardSound, SoundType},
};

#[derive(Clone)]
pub struct Overlay {
    sound: ChessBoardSound,
    pub hints: Vec<ChessMove>,
    pub selected: Option<Square>,
    pub drag: Option<Point>,
    pub highlight: BitBoard,
    pub anchor: Option<Square>,
    pub arrows: Vec<ChessMove>,
}

impl Overlay {
    pub fn new() -> Self {
        Self {
            sound: ChessBoardSound::new(),
            hints: Vec::new(),
            selected: None,
            drag: None,
            highlight: BitBoard::default(),
            anchor: None,
            arrows: Vec::new(),
        }
    }

    fn clear_selection(&mut self) {
        self.hints.clear();
        self.selected = None;
        self.drag = None;
    }

    fn clear_overlay(&mut self) {
        self.arrows.clear();
        self.highlight = BitBoard::new(0);
    }
}

impl Overlay {
    fn cursor_square(bounds: Rectangle, cursor: mouse::Cursor, flipped: bool) -> Option<Square> {
        let pos = cursor.position_in(bounds)?;
        Some(Self::pos_to_square(bounds, pos, flipped))
    }

    fn pos_to_board(bounds: Rectangle, pos: Point, flipped: bool) -> (f32, f32) {
        let size = bounds.width.min(bounds.height);

        let row = if flipped { pos.y } else { size - pos.y };
        let col = if flipped { size - pos.x } else { pos.x };

        ((col / size) * 8.0, (row / size) * 8.0)
    }

    fn board_to_square(col: f32, row: f32) -> Square {
        Square::make_square(
            Rank::from_index(row as usize),
            File::from_index(col as usize),
        )
    }

    fn pos_to_square(bounds: Rectangle, pos: Point, flipped: bool) -> Square {
        let (col, row) = Self::pos_to_board(bounds, pos, flipped);
        Self::board_to_square(col, row)
    }

    fn find_move(&self, sq: Square) -> Option<ChessMove> {
        self.hints.iter().find(|mv| mv.get_dest() == sq).cloned()
    }
}

impl Overlay {
    pub fn on_diff(&mut self, old: &BState, new: &BState) {
        if old.game == new.game {
            return;
        }

        self.clear_selection();
        self.clear_overlay();

        let board = new.game.board;

        if board.status() == BoardStatus::Checkmate {
            self.sound.play(SoundType::GameEnd);
        }

        let sound = if let Some(ant) = new.game.annotation {
            match ant.kind {
                _ if *board.checkers() != BitBoard::new(0) => Some(SoundType::MoveCheck),
                Move::Promotion => Some(SoundType::Promote),
                Move::Capture | Move::EnPassant => Some(SoundType::Capture),
                Move::Castling => Some(SoundType::Castle),
                _ => Some(SoundType::MoveSelf),
            }
        } else {
            Some(SoundType::GameStart)
        };

        if let Some(s) = sound {
            self.sound.play(s);
        }
    }

    pub fn mouse_interaction(
        &self,
        bounds: Rectangle,
        cursor: mouse::Cursor,
        state: &BState,
    ) -> mouse::Interaction {
        if self.drag.is_some() {
            return mouse::Interaction::Grabbing;
        }

        if let Some(sq) = Self::cursor_square(bounds, cursor, state.flipped) {
            if state.game.board.piece_on(sq).is_some() {
                return mouse::Interaction::Grab;
            }
        }

        mouse::Interaction::default()
    }
}

impl Overlay {
    pub fn on_event<Message>(
        &mut self,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
        state: &BState,
        messages: &Messages<Message>,
        caches: &mut Caches,
        shell: &mut Shell<'_, Message>,
    ) {
        match event {
            canvas::Event::Mouse(mouse_event) => {
                self.on_event_mouse(mouse_event, bounds, cursor, state, messages, caches, shell)
            }
            _ => return,
        }
    }

    fn on_event_mouse<Message>(
        &mut self,
        event: &mouse::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
        state: &BState,
        messages: &Messages<Message>,
        caches: &mut Caches,
        shell: &mut Shell<'_, Message>,
    ) {
        match event {
            mouse::Event::ButtonPressed(mouse::Button::Right) => {
                if let Some(sq) = Self::cursor_square(bounds, cursor, state.flipped) {
                    self.anchor = Some(sq);
                    self.clear_selection();

                    caches.board_overlay.clear();
                    shell.request_redraw();
                }
            }
            mouse::Event::ButtonReleased(mouse::Button::Right) => {
                let Some(from_h) = self.anchor else {
                    return;
                };

                if let Some(sq) = Self::cursor_square(bounds, cursor, state.flipped) {
                    if sq == from_h {
                        self.highlight ^= BitBoard::from_square(sq);
                        caches.board_overlay.clear();
                    } else {
                        let arrow = ChessMove::new(from_h, sq, None);
                        if let Some(pos) = self.arrows.iter().position(|mv| *mv == arrow) {
                            self.arrows.remove(pos);
                        } else {
                            self.arrows.push(arrow);
                        }
                        caches.overlay.clear();
                    }
                    shell.request_redraw();
                }
            }
            mouse::Event::ButtonPressed(mouse::Button::Left) => {
                if let Some(pos) = cursor.position_in(bounds) {
                    let (col, row) = Self::pos_to_board(bounds, pos, state.flipped);
                    let pos = Point::new(col, row);
                    let square = Self::board_to_square(col, row);

                    self.clear_overlay();
                    caches.overlay.clear();

                    if let Some(mv) = self.find_move(square) {
                        self.clear_selection();
                        caches.board_overlay.clear();
                        if let Some(on_move) = &messages.on_move {
                            shell.publish((on_move)(mv));
                        }
                        return;
                    }

                    if state.game.board.piece_on(square).is_none() {
                        self.clear_selection();
                        caches.board_overlay.clear();
                        shell.request_redraw();
                        return;
                    }

                    self.hints.clear();
                    self.selected = Some(square);
                    self.drag = Some(pos);

                    if messages.on_move.is_some() {
                        for mv in MoveGen::new_legal(&state.game.board) {
                            if mv.get_source() == square
                                && (mv.get_promotion() == Some(Piece::Queen)
                                    || mv.get_promotion().is_none())
                            {
                                self.hints.push(mv);
                            }
                        }
                    }

                    caches.board_overlay.clear();
                    caches.pieces.clear();
                    caches.drag.clear();
                    shell.request_redraw();
                }
            }
            mouse::Event::ButtonReleased(mouse::Button::Left) => {
                self.drag = None;
                if let Some(sq) = Self::cursor_square(bounds, cursor, state.flipped) {
                    if let Some(mv) = self.find_move(sq) {
                        self.clear_selection();
                        if let Some(on_move) = &messages.on_move {
                            shell.publish((on_move)(mv));
                        }
                    }
                }
                caches.board_overlay.clear();
                caches.pieces.clear();
                caches.drag.clear();
                caches.overlay.clear();
                shell.request_redraw();
            }
            mouse::Event::CursorMoved { position: _ } => {
                if self.drag.is_some() {
                    if let Some(pos) = cursor.position_in(bounds) {
                        let (col, row) = Self::pos_to_board(bounds, pos, state.flipped);
                        self.drag = Some(Point::new(col, row));
                        caches.drag.clear();
                        caches.board_overlay.clear();
                        shell.request_redraw();
                    }
                }
            }
            _ => return,
        }
    }
}
