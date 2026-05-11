use color_eyre::Result;

mod cli;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        return cli::run(&args);
    }

    let terminal = ratatui::init();
    let result = skillweaver::app::run(terminal);
    ratatui::restore();
    result.map_err(Into::into)
}
