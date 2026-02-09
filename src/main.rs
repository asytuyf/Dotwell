mod app;
mod config;
mod installer;
mod scanner;
mod ui;

use app::{App, View};
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, stdout};

fn main() -> Result<()> {
    color_eyre::install()?;

    // Check for CLI flags
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && (args[1] == "--list" || args[1] == "-l") {
        return list_dotfiles();
    }

    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Create app
    let mut app = App::new()?;

    // Run the app
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    result
}

fn list_dotfiles() -> Result<()> {
    use crate::config::get_compiler_name;
    use crate::scanner::DotfileScanner;

    let scanner = DotfileScanner::new();
    let dotfiles = scanner.scan()?;

    if dotfiles.is_empty() {
        println!("No dotfiles found.");
        println!("\nCreate a dotwell.toml or dotwell.json file in your dotfiles directory.");
        return Ok(());
    }

    println!("Found {} dotfile configuration{}:\n",
        dotfiles.len(),
        if dotfiles.len() == 1 { "" } else { "s" });

    for (i, entry) in dotfiles.iter().enumerate() {
        let compiler = get_compiler_name(&entry.config.compiler);
        println!("{}. {} [{}]", i + 1, entry.config.name, compiler);
        println!("   {}", entry.config.description);
        println!("   Path: {}", entry.path.display());
        println!("   Category: {}", entry.config.category);
        println!("   Dependencies: {}", entry.config.dependencies.join(", "));
        println!();
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            ui::render(frame, app);
        })?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match app.view {
                    View::Home => match code {
                        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                        KeyCode::Char('b') => app.go_to_view(View::Browse),
                        _ => {}
                    },
                    View::Browse => match code {
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Esc | KeyCode::Char('b') => app.go_back(),
                        KeyCode::Down | KeyCode::Char('j') => app.next_item(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous_item(),
                        KeyCode::Char('i') => {
                            if let Err(e) = app.install_selected() {
                                eprintln!("Install error: {}", e);
                            }
                        }
                        KeyCode::Enter => {
                            if !app.dotfiles.is_empty() {
                                app.go_to_view(View::Preview);
                            }
                        }
                        _ => {}
                    },
                    View::Preview => match code {
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Esc => app.go_back(),
                        KeyCode::Char('i') | KeyCode::Enter => {
                            if let Err(e) = app.install_selected() {
                                // Show error to user somehow
                                eprintln!("Install error: {}", e);
                            }
                        }
                        _ => {}
                    },
                    View::Installing => match code {
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Esc | KeyCode::Enter => app.go_back(),
                        _ => {}
                    },
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
