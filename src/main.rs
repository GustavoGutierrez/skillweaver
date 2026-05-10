use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = skillweaver::app::run(terminal);
    ratatui::restore();
    result.map_err(Into::into)
}
