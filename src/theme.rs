//! Visual identity for Tincta: "Encre" (ink).
//!
//! A single violet-ink accent against warm ink-black (dark) or paper (light)
//! surfaces. Style helpers take `dark: bool` directly instead of deriving
//! colors from `iced::Theme`, since the two custom themes only carry the
//! base palette and we want exact control over surfaces/borders/hairlines.

use iced::widget::{button, container};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

pub fn ink_dark() -> Theme {
    Theme::custom(
        "Tincta Ink".to_string(),
        iced::theme::Palette {
            background: rgb(0x14, 0x15, 0x1A),
            text: rgb(0xEC, 0xEA, 0xE4),
            primary: ACCENT,
            success: rgb(0x4C, 0xAF, 0x8A),
            danger: rgb(0xE0, 0x6C, 0x5A),
        },
    )
}

pub fn ink_light() -> Theme {
    Theme::custom(
        "Tincta Paper".to_string(),
        iced::theme::Palette {
            background: rgb(0xF7, 0xF5, 0xF0),
            text: rgb(0x1E, 0x1F, 0x25),
            primary: ACCENT,
            success: rgb(0x2E, 0x7D, 0x5B),
            danger: rgb(0xC0, 0x42, 0x2D),
        },
    )
}

pub const ACCENT: Color = Color::from_rgb(
    0x6E as f32 / 255.0,
    0x5B as f32 / 255.0,
    1.0,
);

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

/// Concrete color tokens for one mode (dark or light).
pub struct Palette {
    pub bg: Color,
    pub surface: Color,
    pub elevated: Color,
    pub border: Color,
    pub text: Color,
    pub muted: Color,
    pub accent: Color,
    pub accent_soft: Color,
}

pub fn palette(dark: bool) -> Palette {
    if dark {
        Palette {
            bg: rgb(0x14, 0x15, 0x1A),
            surface: rgb(0x1B, 0x1C, 0x22),
            elevated: rgb(0x21, 0x22, 0x2A),
            border: rgb(0x2E, 0x2F, 0x38),
            text: rgb(0xEC, 0xEA, 0xE4),
            muted: rgb(0x8A, 0x8C, 0x97),
            accent: ACCENT,
            accent_soft: Color { a: 0.16, ..ACCENT },
        }
    } else {
        Palette {
            bg: rgb(0xF7, 0xF5, 0xF0),
            surface: rgb(0xFF, 0xFF, 0xFF),
            elevated: rgb(0xFB, 0xFA, 0xF7),
            border: rgb(0xE3, 0xE0, 0xD8),
            text: rgb(0x1E, 0x1F, 0x25),
            muted: rgb(0x74, 0x73, 0x7C),
            accent: ACCENT,
            accent_soft: Color { a: 0.12, ..ACCENT },
        }
    }
}

/// A flat container with a bottom hairline (menu bar, toolbar, status bar).
pub fn bar(dark: bool) -> impl Fn(&Theme) -> container::Appearance {
    let p = palette(dark);
    move |_theme: &Theme| container::Appearance {
        text_color: Some(p.text),
        background: Some(Background::Color(p.surface)),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
    }
}

/// The sidebar panel background.
pub fn panel(dark: bool) -> impl Fn(&Theme) -> container::Appearance {
    let p = palette(dark);
    move |_theme: &Theme| container::Appearance {
        text_color: Some(p.text),
        background: Some(Background::Color(p.surface)),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
    }
}

/// An elevated card (dropdown menus, the inline file action row).
pub fn card(dark: bool) -> impl Fn(&Theme) -> container::Appearance {
    let p = palette(dark);
    move |_theme: &Theme| container::Appearance {
        text_color: Some(p.text),
        background: Some(Background::Color(p.elevated)),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Shadow {
            color: Color { a: 0.35, ..Color::BLACK },
            offset: Vector::new(0.0, 6.0),
            blur_radius: 18.0,
        },
    }
}

/// A muted "eyebrow" label color (section headers like FICHIERS).
pub fn muted_text(dark: bool) -> Color {
    palette(dark).muted
}

pub fn accent_color() -> Color {
    ACCENT
}

/// Ghost button used in menu bars, toolbars, and menu items: transparent,
/// a soft accent wash when active/open, a subtle surface tint on hover.
pub struct GhostButton {
    pub dark: bool,
    pub active: bool,
}

impl button::StyleSheet for GhostButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let p = palette(self.dark);
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(Background::Color(if self.active {
                p.accent_soft
            } else {
                Color::TRANSPARENT
            })),
            text_color: if self.active { p.accent } else { p.text },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let p = palette(self.dark);
        button::Appearance {
            background: Some(Background::Color(if self.active {
                p.accent_soft
            } else {
                p.elevated
            })),
            ..self.active(style)
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
    }
}

/// A file row in the sidebar: flat, with a left accent bar when selected.
pub struct SidebarRow {
    pub dark: bool,
    pub selected: bool,
}

impl button::StyleSheet for SidebarRow {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let p = palette(self.dark);
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(Background::Color(if self.selected {
                p.accent_soft
            } else {
                Color::TRANSPARENT
            })),
            text_color: if self.selected { p.accent } else { p.text },
            border: Border {
                color: if self.selected { p.accent } else { Color::TRANSPARENT },
                width: if self.selected { 2.0 } else { 0.0 },
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let p = palette(self.dark);
        button::Appearance {
            background: Some(Background::Color(if self.selected {
                p.accent_soft
            } else {
                p.elevated
            })),
            ..self.active(style)
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
    }
}

/// A small destructive-leaning text action (used inside the inline file
/// action row, e.g. "Fermer").
pub struct TextAction {
    pub dark: bool,
    pub enabled: bool,
}

impl button::StyleSheet for TextAction {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let p = palette(self.dark);
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: if self.enabled { p.text } else { p.muted },
            border: Border {
                color: p.border,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let p = palette(self.dark);
        button::Appearance {
            background: Some(Background::Color(if self.enabled {
                p.accent_soft
            } else {
                Color::TRANSPARENT
            })),
            ..self.active(style)
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
    }
}
