pub const RESET_COLOR: &str = "\x1b";

pub enum LogColor {
    BrightBlue,
    BrightGreen,
    BrightRed,
    BrightYellow,
    Blue,
    Green,
    Red,
    Yellow,
    White,
    Grey,
}

impl ToString for LogColor {
    fn to_string(&self) -> String {
        match self {
            LogColor::Green => format!("{RESET_COLOR}[32m"),
            LogColor::BrightGreen => format!("{RESET_COLOR}[32;1m"),
            LogColor::Blue => format!("{RESET_COLOR}[34m"),
            LogColor::BrightBlue => format!("{RESET_COLOR}[34;1m"),
            LogColor::Yellow => format!("{RESET_COLOR}[33m"),
            LogColor::BrightYellow => format!("{RESET_COLOR}[33;1m"),
            LogColor::Red => format!("{RESET_COLOR}[31m"),
            LogColor::BrightRed => format!("{RESET_COLOR}[31;1m"),
            LogColor::White => format!("{RESET_COLOR}[37;0m"),
            LogColor::Grey => format!("{RESET_COLOR}[37;2m"),
        }
    }
}
