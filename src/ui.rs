use crate::app::{App, View};
use crate::config::get_compiler_name;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App) {
    match app.view {
        View::Home => render_home(frame, app),
        View::Browse => render_browse(frame, app),
        View::Preview => render_preview(frame, app),
        View::Installing => render_installing(frame, app),
    }
}

fn render_home(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12), // Logo
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    render_logo(frame, chunks[0]);

    let stats = format!(
        "Found {} dotfile configuration{}",
        app.dotfiles.len(),
        if app.dotfiles.len() == 1 { "" } else { "s" }
    );

    let content = vec![
        Line::from(""),
        Line::from(stats).centered(),
        Line::from("").centered(),
        Line::from("Press 'b' to browse dotfiles").centered(),
        Line::from("Press 'q' to quit").centered(),
    ];

    let content_widget = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(" Welcome "));

    frame.render_widget(content_widget, chunks[1]);
    render_footer(frame, chunks[2], "Home");
}

fn render_browse(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(area);

    if app.dotfiles.is_empty() {
        let msg = Paragraph::new("No dotfiles found.\n\nCreate a dotwell.toml or dotwell.json file in your dotfiles directory.")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title(" Browse Dotfiles "));
        frame.render_widget(msg, chunks[0]);
    } else {
        let items: Vec<ListItem> = app
            .dotfiles
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let compiler = get_compiler_name(&entry.config.compiler);

                // Add indent for theme category items
                let indent = if entry.config.category == "themes" {
                    "    "
                } else {
                    "  "
                };

                let icon = if entry.config.category == "themes" {
                    "🎨 "
                } else {
                    "📦 "
                };

                let content = Line::from(vec![
                    Span::raw(indent),
                    Span::styled(icon, Style::default()),
                    Span::styled(
                        format!("{} ", entry.config.name),
                        Style::default().fg(Color::Cyan).bold(),
                    ),
                    Span::styled(
                        format!("[{}] ", compiler),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(
                        format!("{}", entry.config.description),
                        Style::default().fg(Color::Gray),
                    ),
                ]);

                let style = if i == app.selected_index {
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Browse Dotfiles ")
                .title_bottom(" ↑/↓: navigate | Enter: preview | i: install | Esc/b: back | q: quit "),
        );

        frame.render_widget(list, chunks[0]);
    }

    render_footer(frame, chunks[1], "Browse");
}

fn render_preview(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(area);

    if let Some(entry) = app.selected_dotfile() {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(chunks[0]);

        // Left panel: metadata
        let compiler = get_compiler_name(&entry.config.compiler);
        let metadata = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Name: ", Style::default().fg(Color::Yellow).bold()),
                Span::raw(&entry.config.name),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Category: ", Style::default().fg(Color::Yellow).bold()),
                Span::raw(&entry.config.category),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Compiler: ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(compiler, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Path: ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(
                    entry.path.display().to_string(),
                    Style::default().fg(Color::Gray),
                ),
            ]),
            Line::from(""),
            Line::from(
                Span::styled("Description:", Style::default().fg(Color::Yellow).bold()),
            ),
            Line::from(format!("  {}", entry.config.description)),
            Line::from(""),
            Line::from(
                Span::styled("Dependencies:", Style::default().fg(Color::Yellow).bold()),
            ),
        ];

        let mut metadata_lines = metadata;
        for dep in &entry.config.dependencies {
            metadata_lines.push(Line::from(format!("  • {}", dep)));
        }

        let metadata_widget = Paragraph::new(metadata_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Metadata ")
                    .title_bottom(" Press 'i' or Enter to install "),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(metadata_widget, main_chunks[0]);

        // Right panel: files
        let mut files_lines = vec![Line::from("")];
        for file in &entry.config.files {
            files_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("📄 ", Style::default().fg(Color::Blue)),
                Span::raw(file),
            ]));
        }

        let files_widget = Paragraph::new(files_lines)
            .block(Block::default().borders(Borders::ALL).title(" Files "));

        frame.render_widget(files_widget, main_chunks[1]);
    }

    render_footer(frame, chunks[1], "Preview");
}

fn render_installing(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(area);

    let status_icon = if app.install_success {
        Span::styled("✓", Style::default().fg(Color::Green).bold())
    } else {
        Span::styled("✗", Style::default().fg(Color::Red).bold())
    };

    let status_text = if app.install_success {
        "Installation completed successfully!"
    } else {
        "Installation failed!"
    };

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            status_icon,
            Span::raw("  "),
            Span::styled(
                status_text,
                Style::default().fg(if app.install_success {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
        ]),
        Line::from(""),
        Line::from(
            Span::styled("Output:", Style::default().fg(Color::Yellow).bold()),
        ),
        Line::from(""),
    ];

    if let Some(output) = &app.install_output {
        for line in output.lines() {
            lines.push(Line::from(format!("  {}", line)));
        }
    }

    let content = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Installation Result ")
                .title_bottom(" Press Enter or Esc to go back "),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(content, chunks[0]);
    render_footer(frame, chunks[1], "Installing");
}

fn render_logo(frame: &mut Frame, area: Rect) {
    let logo = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("           ██████╗  ██████╗ ████████╗", Style::default().fg(Color::Cyan).bold()),
        ]),
        Line::from(vec![
            Span::styled("           ██╔══██╗██╔═══██╗╚══██╔══╝", Style::default().fg(Color::Cyan).bold()),
        ]),
        Line::from(vec![
            Span::styled("           ██║  ██║██║   ██║   ██║   ", Style::default().fg(Color::Cyan).bold()),
        ]),
        Line::from(vec![
            Span::styled("           ██║  ██║██║   ██║   ██║   ", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("           ██████╔╝╚██████╔╝   ██║   ", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("           ╚═════╝  ╚═════╝    ╚═╝   ", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("                    WELL ", Style::default().fg(Color::Blue).bold()),
            Span::styled("❄", Style::default().fg(Color::LightBlue).bold()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "         NixOS Dotfiles Manager",
                Style::default().fg(Color::Gray),
            ),
            Span::raw("  "),
            Span::styled("v0.1.0", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let logo_widget = Paragraph::new(logo)
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(logo_widget, area);
}

fn render_footer(frame: &mut Frame, area: Rect, view_name: &str) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" View: ", Style::default().fg(Color::DarkGray)),
        Span::styled(view_name, Style::default().fg(Color::Cyan).bold()),
        Span::styled(" | ", Style::default().fg(Color::DarkGray)),
        Span::styled("q", Style::default().fg(Color::Yellow)),
        Span::styled(": quit ", Style::default().fg(Color::DarkGray)),
    ]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}
