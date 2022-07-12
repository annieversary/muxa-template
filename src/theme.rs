use muxa::theme::ThemeTrait;

#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Dark,
    Light,
    Pink,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

impl ThemeTrait for Theme {
    fn css_url(&self) -> &'static str {
        match self {
            Theme::Dark => "/css/theme-dark.css",
            Theme::Light => "/css/theme-light.css",
            Theme::Pink => "/css/theme-pink.css",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "dark" => Some(Theme::Dark),
            "light" => Some(Theme::Light),
            "pink" => Some(Theme::Pink),
            _ => None,
        }
    }
}
