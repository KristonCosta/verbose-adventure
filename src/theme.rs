
pub mod theme {
    use console_backend::{
        Color,
        colors,
    };
    // TODO: MOVE THIS INTO A CONFIG
    lazy_static! {
        pub static ref BACKGROUND: Color = *colors::BLACK;
        pub static ref COLOR_DARK_WALL: Color = *colors::NAVY;
        pub static ref COLOR_DARK_FLOOR: Color = *colors::DARK_INDIGO;
        pub static ref RED_ALERT_TEXT: Color = *colors::DARK_RED;
        pub static ref GREEN_ALERT_TEXT: Color = *colors::DARK_GREEN;
        pub static ref REGULAR_ALERT_TEXT: Color = *colors::WHITE;
        pub static ref PLAYER: Color = *colors::WHITE;
        pub static ref BLOOD: Color = *colors::RED;
        pub static ref TROLL: Color = *colors::DESATURATED_GREEN;
        pub static ref ORC: Color = *colors::LIGHT_GREEN;
        pub static ref HEALING_ITEM: Color = *colors::LIGHT_SLATE_BLUE;
    }
}