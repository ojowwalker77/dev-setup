use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::{
    io,
    time::Duration,
};

use crate::commands;
use crate::config::Config;

pub struct App {
    pub should_quit: bool,
    pub menu_state: ListState,
    pub ai_enabled: bool,

    pub status_message: Option<String>,
    pub loading: bool,
    pub menu_items: Vec<MenuItem>,
}

#[derive(Clone)]
pub struct MenuItem {
    pub id: String,
    pub title: String,
}

impl App {
    pub async fn new() -> Result<Self> {
        let config = Config::load().await?;
        let ai_enabled = config.gemini_api_key.is_some();

        let mut menu_items = vec![
            MenuItem {
                id: "start".to_string(),
                title: "Start development server".to_string(),
            },
            MenuItem {
                id: "build".to_string(),
                title: "Run build and type check".to_string(),
            },
            MenuItem {
                id: "push".to_string(),
                title: "Commit and push changes".to_string(),
            },
            MenuItem {
                id: "lint".to_string(),
                title: "Run linting only".to_string(),
            },
            MenuItem {
                id: "typecheck".to_string(),
                title: "Run type check only".to_string(),
            },
            MenuItem {
                id: "ai_debug".to_string(),
                title: if ai_enabled { "AI Debug Mode" } else { "Setup AI Error Fixing" }.to_string(),
            },
        ];

        if ai_enabled {
            menu_items.push(MenuItem {
                id: "ai_config".to_string(),
                title: "Reconfigure AI settings".to_string(),
            });
        }

        let mut state = ListState::default();
        state.select(Some(0));

        Ok(Self {
            should_quit: false,
            menu_state: state,
            ai_enabled,
            status_message: None,
            loading: false,
            menu_items,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run_app(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err)
        }

        Ok(())
    }

    async fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                self.should_quit = true;
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                self.next_item();
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                self.previous_item();
                            }
                            KeyCode::Enter => {
                                self.execute_selected_item().await?;
                            }
                            _ => {}
                        }
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn next_item(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i >= self.menu_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    fn previous_item(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.menu_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    async fn execute_selected_item(&mut self) -> Result<()> {
        if let Some(i) = self.menu_state.selected() {
            if let Some(item) = self.menu_items.get(i) {
                self.loading = true;
                self.status_message = Some(format!("Executing: {}", item.title));

                match item.id.as_str() {
                    "start" => {
                        self.should_quit = true;
                        commands::execute_command(crate::cli::Commands::Start).await?;
                    }
                    "build" => {
                        commands::execute_command(crate::cli::Commands::Build).await?;
                    }
                    "push" => {
                        commands::execute_command(crate::cli::Commands::Push).await?;
                    }
                    "lint" => {
                        commands::execute_command(crate::cli::Commands::Lint).await?;
                    }
                    "typecheck" => {
                        commands::execute_command(crate::cli::Commands::TypeCheck).await?;
                    }
                    "ai_debug" => {
                        if self.ai_enabled {
                            commands::execute_command(crate::cli::Commands::Analyze { error: None }).await?;
                        } else {
                            commands::execute_command(crate::cli::Commands::Setup).await?;
                        }
                    }
                    "ai_config" => {
                        commands::execute_command(crate::cli::Commands::Setup).await?;
                    }
                    _ => {}
                }

                self.loading = false;
                self.status_message = Some(format!("✓ {}", item.title));
            }
        }
        Ok(())
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area());

        // Header
        let header = Paragraph::new("🚀 Dev Tools CLI")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Menu
        let items: Vec<ListItem> = self
            .menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if Some(i) == self.menu_state.selected() {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else {
                    Style::default()
                };

                let content = Line::from(vec![
                    Span::styled(format!("{}. ", i + 1), Style::default().fg(Color::Yellow)),
                    Span::styled(&item.title, style),
                ]);

                ListItem::new(content)
            })
            .collect();

        let menu_block = Block::default()
            .title("Development Actions")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));

        let menu = List::new(items)
            .block(menu_block)
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(menu, chunks[1], &mut self.menu_state);

        // Footer
        let footer_text = if let Some(msg) = &self.status_message {
            msg.clone()
        } else {
            "Use ↑↓ or j/k to navigate, Enter to select, q to quit".to_string()
        };

        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[2]);

        // Loading indicator
        if self.loading {
            let popup_area = centered_rect(50, 20, f.area());
            f.render_widget(Clear, popup_area);
            let loading_block = Block::default()
                .title("Processing...")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(loading_block, popup_area);
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}