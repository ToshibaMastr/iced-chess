use iced::{Color, Theme};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Button {
    pub background: Color,
    pub font: Color,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Style {
    pub button: Button,
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
        button: Button {
            background: Color::from_rgb8(54, 52, 52),
            font: Color::from_rgb8(198, 197, 197),
        },
    }
}
