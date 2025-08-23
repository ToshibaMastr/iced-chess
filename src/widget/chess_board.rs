pub mod overlay;
pub mod render;
pub mod sound;

use chess::ChessMove;

use iced::{
    Element, Event, Length, Point, Rectangle, Renderer, Size,
    advanced::{
        Clipboard, Layout, Renderer as _, Shell, Widget,
        graphics::geometry::Renderer as _,
        layout::{Limits, Node},
        renderer,
        widget::{
            Tree,
            tree::{self, Tag},
        },
    },
    mouse::{self, Cursor},
    widget::canvas::Cache,
};

use crate::{chess::GameState, style::chess_board::Catalog};

use overlay::Overlay;
use render::ChessBoardRenderer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BState {
    pub game: GameState,
    pub flipped: bool,
}

pub struct Messages<Message> {
    on_move: Option<Box<dyn Fn(ChessMove) -> Message>>,
}

pub struct ChessBoard<Message, Theme: Catalog> {
    width: Length,
    height: Length,
    class: Theme::Class<'static>,
    state: BState,
    message: Messages<Message>,
}

impl<Message, Theme> ChessBoard<Message, Theme>
where
    Theme: Catalog,
{
    pub fn new(game: GameState, flipped: bool) -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            class: Theme::default(),
            state: BState { game, flipped },
            message: Messages { on_move: None },
        }
    }

    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    #[must_use]
    pub fn on_move_maybe<F>(mut self, on_move: Option<F>) -> Self
    where
        F: 'static + Fn(ChessMove) -> Message,
    {
        self.message.on_move = on_move.map(|f| Box::new(f) as Box<dyn Fn(ChessMove) -> Message>);
        self
    }
}

impl<'a, Message, Theme> Widget<Message, Theme, Renderer> for ChessBoard<Message, Theme>
where
    Message: 'a + Clone,
    Theme: Catalog,
{
    fn tag(&self) -> Tag {
        Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.state))
    }

    fn diff(&self, tree: &mut Tree) {
        let wstate: &mut State = tree.state.downcast_mut();

        if self.state != wstate.state {
            wstate.cache.board.clear();
            wstate.cache.board.clear();
            wstate.cache.pieces.clear();
            wstate.cache.overlay.clear();

            wstate.overlay.on_diff(&wstate.state, &self.state);

            wstate.state = self.state;
        }
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        let resolved = limits.resolve(self.width, self.height, Size::ZERO);
        let side = resolved.width.min(resolved.height);
        Node::new(Size::new(side, side))
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let wstate: &State = state.state.downcast_ref();

        wstate
            .overlay
            .mouse_interaction(bounds, cursor, &wstate.state)
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let wstate: &mut State = state.state.downcast_mut();

        wstate.overlay.on_event(
            event,
            bounds,
            cursor,
            &self.state,
            &self.message,
            &mut wstate.cache,
            shell,
        );
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let wstate: &State = state.state.downcast_ref();
        let style = theme.style(&self.class);

        let cbrenderer = ChessBoardRenderer::new(style, self.state, bounds);

        let geometrys = vec![
            wstate.cache.board.draw(renderer, bounds.size(), |frame| {
                cbrenderer.draw_board(frame, &wstate.overlay);
            }),
            wstate
                .cache
                .board_overlay
                .draw(renderer, bounds.size(), |frame| {
                    cbrenderer.draw_board_overlay(frame, &wstate.overlay);
                }),
            wstate.cache.pieces.draw(renderer, bounds.size(), |frame| {
                cbrenderer.draw_pieces(frame, &wstate.overlay);
            }),
            wstate.cache.drag.draw(renderer, bounds.size(), |frame| {
                cbrenderer.draw_drag(frame, &wstate.overlay);
            }),
            wstate.cache.overlay.draw(renderer, bounds.size(), |frame| {
                cbrenderer.draw_arrows(frame, &wstate.overlay);
            }),
        ];

        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            for gm in geometrys {
                renderer.draw_geometry(gm);
            }
        });
    }
}

#[derive(Default)]
pub struct Caches {
    pub(crate) board: Cache,
    pub(crate) board_overlay: Cache,
    pub(crate) pieces: Cache,
    pub(crate) drag: Cache,
    pub(crate) overlay: Cache,
}

pub struct State {
    pub(crate) overlay: Overlay,
    pub(crate) cache: Caches,
    pub(crate) state: BState,
}

impl State {
    pub fn new(state: BState) -> Self {
        Self {
            overlay: Overlay::new(),
            cache: Caches::default(),
            state,
        }
    }
}

impl<'a, Message, Theme> From<ChessBoard<Message, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Theme: 'a + Catalog,
    Message: Clone + 'a,
{
    fn from(candle_charts: ChessBoard<Message, Theme>) -> Self {
        Element::new(candle_charts)
    }
}
