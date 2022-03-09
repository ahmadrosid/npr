mod document;

use std::{
    cmp::{max, min},
    process::Command,
};
use tuikit::prelude::*;

use document::Document;

fn main() {
    let term: Term<()> = Term::new().unwrap();
    let mut row = 1;
    let mut query = String::new();
    let doc = Document::parse_script();
    if let Err(ref err) = doc {
        let attr = Attr {
            fg: Color::RED,
            ..Attr::default()
        };
        let _ = term.print_with_attr(row, 0, &err, attr);
    };

    let mut data = doc.as_ref().unwrap().search(&query);
    let mut max_height = data.len();

    while let Ok(ev) = term.poll_event() {
        match ev {
            Event::Key(Key::ESC) => break,
            Event::Key(Key::Enter) => {
                if let Some(key) = data.get(row - 1) {
                    if let Some(script) = doc.as_ref().unwrap().get_script(key) {
                        let child = Command::new("npm").arg("run").arg(script).spawn();
                        let _ = child.unwrap();
                    }
                }
                break;
            }
            Event::Key(Key::Up) => row = max(row - 1, 1),
            Event::Key(Key::Down) => row = min(row + 1, max_height),
            Event::Key(Key::Char(ch)) => query.push(ch),
            Event::Key(Key::Backspace) => {
                if !query.is_empty() {
                    query.truncate(query.len() - 1)
                }
            }
            _ => {}
        }

        data = doc.as_ref().unwrap().search(&query);
        max_height = data.len();
        let _ = term.clear();
        for (i, v) in data.iter().enumerate() {
            let _ = term.print(i + 1, 0, v);
        }

        let display_query = format!("> {}", query);
        let _ = term.print(0, 0, &display_query);
        let _ = term.set_cursor(0, display_query.len());

        if let Some(val) = data.get(row - 1) {
            let attr = Attr {
                bg: Color::LIGHT_BLACK,
                ..Attr::default()
            };
            let _ = term.print_with_attr(row, 0, &*val, attr);
        } else if data.is_empty() {
            let _ = term.clear();
            let display_query = format!("> {}", query);
            let _ = term.print(0, 0, &display_query);
            let _ = term.set_cursor(0, display_query.len());
            let _ = term.print(1, 0, &format!("Query '{}' not found!", query));
        }
        let _ = term.present();
    }
}
