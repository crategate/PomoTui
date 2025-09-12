use crate::app::App;

pub mod app;
pub mod event;
pub mod ui;

use ratatui::{TerminalOptions, Viewport};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(3),
    });
    color_eyre::install()?;
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
