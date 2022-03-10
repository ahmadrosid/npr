mod app;
mod document;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

use app::App;
use document::Document;

fn main() -> Result<(), Box<dyn Error>> {
    let doc = Document::parse_script();
    if let Err(ref err) = doc {
        println!("{}", err);
        std::process::exit(1);
    };
    let doc = doc.unwrap();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let scripts = doc.search("");
    let app = App::new(scripts, doc);
    let res = app::run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match res {
        Ok(app) => app.run_script(),
        Err(err) => println!("{:?}", err),
    }

    Ok(())
}
