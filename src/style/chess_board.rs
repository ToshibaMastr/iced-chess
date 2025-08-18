use iced::{Color, Theme};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BoardStyle {
    pub light: Color,
    pub dark: Color,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct OverlayStyle {
    pub selected: Color,
    pub prev_move: Color,
    pub drag: Color,
    pub hover: Color,
    pub highlight: Color,
    pub arrow: Color,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Style {
    pub board: BoardStyle,
    pub overlay: OverlayStyle,
}

pub trait Catalog {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(_theme: &Theme) -> Style {
    Style {
        board: BoardStyle {
            light: Color::from_rgb8(235, 236, 208),
            dark: Color::from_rgb8(115, 149, 82),
        },
        overlay: OverlayStyle {
            prev_move: Color::from_rgba8(255, 255, 51, 0.5),
            selected: Color::from_rgba8(255, 255, 51, 0.5),
            drag: Color::from_rgba8(0, 0, 0, 0.14),
            hover: Color::from_rgba8(255, 255, 255, 0.65),
            highlight: Color::from_rgba8(235, 97, 80, 0.8),
            arrow: Color::from_rgba8(255, 170, 0, 0.64),
        },
    }
}
