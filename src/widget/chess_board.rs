pub mod overlay;
pub mod render;

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

use crate::{
    chess::{BoardRole, GameState},
    style::chess_board::Catalog,
};

use overlay::Overlay;
use render::ChessBoardRenderer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BState {
    pub game: GameState,
    pub role: BoardRole,
    pub flipped: bool,
}

pub struct ChessBoard<Message, Theme: Catalog> {
    width: Length,
    height: Length,
    class: Theme::Class<'static>,
    state: BState,
    on_move: Box<dyn Fn(ChessMove) -> Message>,
}

impl<Message, Theme> ChessBoard<Message, Theme>
where
    Theme: Catalog,
{
    pub fn new<F>(game: GameState, flipped: bool, role: BoardRole, on_move: F) -> Self
    where
        F: 'static + Fn(ChessMove) -> Message,
    {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            class: Theme::default(),
            state: BState {
                game,
                role,
                flipped,
            },
            on_move: Box::new(on_move),
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
        tree::State::new(State::new())
    }

    fn diff(&self, tree: &mut Tree) {
        let wstate: &mut State = tree.state.downcast_mut();

        if self.state != wstate.state {
            wstate.board_cache.clear();
            wstate.pieces_cache.clear();
            wstate.overlay_cache.clear();

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

        // shell.request_redraw_at(window::RedrawRequest::NextFrame);
        // match event {
        //     Event::Window(window::Event::RedrawRequested(now)) => {
        //         println!("{:?}", now);
        //     }
        //     _ => {}
        // }

        let event = wstate.overlay.on_event(event, bounds, cursor, &self.state);

        if event != overlay::Event::None {
            // println!("{:?}", event);
        }

        match event {
            overlay::Event::Move(mv) => {
                shell.publish((self.on_move)(mv));
                wstate.board_cache.clear();
                wstate.pieces_cache.clear();
                wstate.overlay_cache.clear();
                shell.request_redraw();
            }
            overlay::Event::Redraw(bc, pc, oc) => {
                if bc {
                    wstate.board_cache.clear();
                }
                if pc {
                    wstate.pieces_cache.clear();
                }
                if oc {
                    wstate.overlay_cache.clear();
                }
                shell.request_redraw();
            }
            overlay::Event::None => {}
        }
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

        let board_geometry = wstate.board_cache.draw(renderer, bounds.size(), |frame| {
            cbrenderer.draw_board(frame, &wstate.overlay);
            cbrenderer.draw_board_overlay(frame, &wstate.overlay);
        });

        let pieces_geometry = wstate.pieces_cache.draw(renderer, bounds.size(), |frame| {
            cbrenderer.draw_pieces(frame, &wstate.overlay);
            cbrenderer.draw_drag(frame, &wstate.overlay);
        });

        let overlay_geometry = wstate.overlay_cache.draw(renderer, bounds.size(), |frame| {
            cbrenderer.draw_arrows(frame, &wstate.overlay);
        });

        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            renderer.draw_geometry(board_geometry);
            renderer.draw_geometry(pieces_geometry);
            renderer.draw_geometry(overlay_geometry);
        });
    }
}

pub struct State {
    pub(crate) overlay: Overlay,
    pub(crate) board_cache: Cache,
    pub(crate) pieces_cache: Cache,
    pub(crate) overlay_cache: Cache,

    pub(crate) state: BState,
}

impl State {
    pub fn new() -> Self {
        Self {
            overlay: Overlay::new(),
            board_cache: Cache::default(),
            pieces_cache: Cache::default(),
            overlay_cache: Cache::default(),

            state: BState {
                game: GameState::new(),
                role: BoardRole::Spectator,
                flipped: false,
            },
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
