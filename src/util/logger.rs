use env_logger::{Builder, fmt::Color};
use log::{LevelFilter, Level};
use std::io::Write;

pub fn create_colored_logger() -> Builder {
    let mut builder = Builder::from_default_env();
    builder
        .format(|buf, record| {
            let level_color = match record.level() {
                Level::Trace => Color::White,
                Level::Debug => Color::Blue,
                Level::Info => Color::Green,
                Level::Warn => Color::Yellow,
                Level::Error => Color::Red,
            };
            let mut level_style = buf.style();
            level_style.set_color(level_color);
            writeln!(
                buf,
                "[{}]: {}",
                level_style.value(record.level()),
                record.args()
            )
        });
    builder
}