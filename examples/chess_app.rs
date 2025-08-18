#![windows_subsystem = "windows"]

use chess::ChessMove;
use iced::{
    Alignment, Element, Task, Theme,
    widget::{button, column, horizontal_space, row, text},
};
use iced_chess::{chess::BoardRole, chess::GameState, widget::ChessBoard};

fn main() -> iced::Result {
    iced::application(ChessApp::new, ChessApp::update, ChessApp::view)
        .title(ChessApp::title)
        .theme(ChessApp::theme)
        .decorations(true)
        .antialiasing(true)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    OnMove(ChessMove),
    Flip,
    Restart,

    Back,
    Next,
    Set(usize),
}

#[derive(Debug)]
struct ChessApp {
    history: Vec<GameState>,
    current: usize,
    flipped: bool,
}

impl ChessApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                history: vec![GameState::new()],
                current: 0,
                flipped: false,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        "O - O".into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: self::Message) {
        match message {
            Message::OnMove(mv) => {
                let state = self.history[self.current].make_move(mv);

                self.history.truncate(self.current + 1);
                self.history.push(state);

                self.current += 1;
            }
            Message::Flip => {
                self.flipped = !self.flipped;
            }
            Message::Restart => {
                self.history = vec![GameState::new()];
                self.current = 0;
            }
            Message::Next => {
                if self.current + 1 < self.history.len() {
                    self.current += 1;
                }
            }
            Message::Back => {
                if self.current > 0 {
                    self.current -= 1;
                }
            }
            Message::Set(current) => {
                if current < self.history.len() {
                    self.current = current;
                }
            }
        }
    }

    fn view(&self) -> Element<'_, self::Message> {
        let game = self.history[self.current];

        let can_go_back = self.current > 0;
        let can_go_next = self.current + 1 < self.history.len();

        let role = if can_go_next {
            BoardRole::Spectator
        } else {
            BoardRole::Analyst
        };

        let chessboard = ChessBoard::new(game, self.flipped, role, Message::OnMove);

        let manag = row![
            button("Flip").on_press(Message::Flip),
            button("Restart").on_press(Message::Restart),
            button("|<").on_press_maybe(can_go_back.then_some(Message::Set(0))),
            button("<").on_press_maybe(can_go_back.then_some(Message::Back)),
            button(">").on_press_maybe(can_go_next.then_some(Message::Next)),
            button(">|")
                .on_press_maybe(can_go_next.then_some(Message::Set(self.history.len() - 1))),
            horizontal_space(),
            text(format!(
                "{} | {:?} | {:?}",
                self.current,
                game.board.status(),
                game.board.side_to_move()
            ))
        ]
        .align_y(Alignment::Center)
        .spacing(10);

        column![chessboard, manag]
            .align_x(Alignment::Center)
            .spacing(10)
            .padding(10)
            .into()
    }
}
