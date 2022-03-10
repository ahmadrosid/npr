use crate::document::Document;
use crossterm::event::{self, Event, KeyCode};
use std::{io, process::Command};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

pub struct App {
    should_run_script: bool,
    index: usize,
    input: String,
    messages: Vec<String>,
    doc: Document,
}

impl App {
    pub fn new(messages: Vec<String>, doc: Document) -> Self {
        Self {
            should_run_script: false,
            index: 0,
            input: String::new(),
            messages,
            doc,
        }
    }

    pub fn run_search(&mut self) {
        self.messages = self.doc.search(&self.input);
    }

    pub fn run_script(&self) {
        if !self.should_run_script {
            return;
        }

        if let Some(key) = self.messages.get(self.index) {
            if let Some(script) = self.doc.get_script(key) {
                let mut child = Command::new("npm").arg("run").arg(script).spawn().unwrap();
                assert!(child.wait().unwrap().success());
            }
        }
    }

    fn udpate_index(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up => {
                if self.index == 0 {
                    return;
                }
                self.index -= 1;
            }
            KeyCode::Down => {
                if self.index == self.messages.len() - 1 {
                    return;
                }
                self.index += 1;
            }
            _ => {}
        };
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<App> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    app.should_run_script = true;
                    return Ok(app);
                }
                KeyCode::Char(c) => {
                    app.input.push(c);
                    app.run_search();
                }
                KeyCode::Backspace => {
                    app.input.pop();
                    app.run_search();
                }
                KeyCode::Up | KeyCode::Down => {
                    app.udpate_index(key.code);
                }
                KeyCode::Esc => {
                    return Ok(app);
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let msg = vec![
        Span::raw("Press "),
        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to close sesssion, "),
        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to execute script"),
    ];

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(Style::default());
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(input, chunks[1]);
    f.set_cursor(chunks[1].x + app.input.width() as u16 + 1, chunks[1].y + 1);

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(m))];
            if i == app.index {
                ListItem::new(content).style(Style::default().bg(Color::Gray).fg(Color::Black))
            } else {
                ListItem::new(content)
            }
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Result"));
    f.render_widget(messages, chunks[2]);
}
