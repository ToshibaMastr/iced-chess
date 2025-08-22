use chess::{Board, ChessMove, Color, Piece};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BoardRole {
    Player(Color),
    Analyst,
    Spectator,
}

impl BoardRole {
    pub fn can_move(&self, color: &Color) -> bool {
        match self {
            BoardRole::Player(clr) => color == clr,
            BoardRole::Analyst => true,
            BoardRole::Spectator => false,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct GameState {
    pub board: Board,
    pub annotation: Option<Annotation>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Annotation {
    pub mv: ChessMove,
    pub kind: Move,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Normal,
    Capture,
    EnPassant,
    Castling,
    Promotion,
}

impl GameState {
    pub fn make_move(&mut self, mv: ChessMove) -> GameState {
        let board = self.board;
        let new_board = self.board.make_move_new(mv);

        let source = mv.get_source();
        let dest = mv.get_dest();

        let color = board.side_to_move();
        let piece = board.piece_on(source);

        let kind = if mv.get_promotion().is_some() {
            Move::Promotion
        } else if board.piece_on(dest).is_some() {
            Move::Capture
        } else if piece == Some(Piece::King)
            && (source.get_file().to_index() as i32 - dest.get_file().to_index() as i32).abs() == 2
        {
            Move::Castling
        } else if piece == Some(Piece::Pawn) && self.board.en_passant() == dest.backward(color) {
            Move::EnPassant
        } else {
            Move::Normal
        };

        GameState {
            board: new_board,
            annotation: Some(Annotation { mv, kind }),
        }
    }
}
