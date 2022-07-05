use crate::app;
use crate::ui;
use app::Mode;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use glob::glob;

use std::{error::Error, io, process::Command, time::Duration};

use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = app::App::new();
    let res = run_app(&mut terminal, &mut app);
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        panic!("run_app returned Error Value, {}", err);
    }
    Ok(())
}
pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: &mut app::App,
) -> Result<(), Box<dyn Error>> {
    app.window_size = terminal::size().unwrap();
    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::CommandMode => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('f') => {
                        app.current_selection += 1;
                    }
                    KeyCode::Char('u') => {
                        let files = glob("*").expect("Failed to read glob pattern");
                        for entry in files {
                            app.files
                                .push(entry.unwrap().into_os_string().into_string().unwrap());
                        }
                        //                        let files: Vec<u8> = Command::new("ls").output().unwrap().stdout;
                        //                        let files = std::str::from_utf8(&files).unwrap();
                        //                        //&Command::new("ls").output().expect("Ls Failed").stdout,
                        //                        app.files.push(files.to_string());
                    }
                    KeyCode::Char('i') => {
                        app.mode = Mode::InputMode;
                    }
                    _ => (),
                },
                Mode::InputMode => match key.code {
                    KeyCode::Char(a) => app.input_buf.push(a.to_string()),
                    _ => (),
                },
            }
        }
    }
}
