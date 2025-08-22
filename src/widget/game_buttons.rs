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

use crate::style::game_buttons::Catalog;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BState {
    pub current: usize,
    pub len: usize,
}

pub struct GameButtons<Message, Theme: Catalog> {
    width: Length,
    height: Length,
    class: Theme::Class<'static>,
    state: BState,

    set: Box<dyn Fn(ChessMove) -> Message>,
    next: Message,
    back: Message,
}

impl<Message, Theme> GameButtons<Message, Theme>
where
    Theme: Catalog,
{
    pub fn new<F>(current: usize, len: usize, set: F, next: Message, back: Message) -> Self
    where
        F: 'static + Fn(ChessMove) -> Message,
    {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            class: Theme::default(),
            state: BState { current, len },
            set: Box::new(set),
            next: next,
            back: back,
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

impl<'a, Message, Theme> Widget<Message, Theme, Renderer> for GameButtons<Message, Theme>
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

    fn diff(&mut self, tree: &mut Tree) {
        let wstate: &mut State = tree.state.downcast_mut();

        if self.state != wstate.state {
            wstate.cache.clear();

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

        mouse::Interaction::default()
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
    }
}

pub struct State {
    pub(crate) cache: Cache,
    pub(crate) state: BState,
}

impl State {
    pub fn new() -> Self {
        Self {
            cache: Cache::default(),
            state: BState { current: 0, len: 0 },
        }
    }
}

impl<'a, Message, Theme> From<GameButtons<Message, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Theme: 'a + Catalog,
    Message: Clone + 'a,
{
    fn from(candle_charts: GameButtons<Message, Theme>) -> Self {
        Element::new(candle_charts)
    }
}
