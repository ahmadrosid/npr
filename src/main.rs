mod document;

use document::Document;
use termion::color;

pub fn main() {
    let query = std::env::args().nth(1).unwrap_or("".to_string());
    let doc = Document::parse_script();
    if let Err(ref err) = doc {
        print!("{}", color::Fg(color::Red));
        println!("{}", err);
    }
    doc.unwrap().search(&query);
}
