use serde_json::{Map, Result, Value};
use std::collections::HashMap;
use std::result::Result as ResultError;
use std::{fs::File, path::Path};

use fuzzy_matcher::clangd::ClangdMatcher;
use fuzzy_matcher::FuzzyMatcher;
use termion::style::{Bold, Reset};

#[cfg(not(feature = "compact"))]
type IndexType = usize;
#[cfg(feature = "compact")]
type IndexType = u32;

#[derive(Default)]
pub struct Document {
    pub scripts: HashMap<String, String>,
    pub matcher: ClangdMatcher,
}

impl Document {
    pub fn parse_script() -> ResultError<Document, String> {
        let path = "package.json";
        if !Path::new(path).exists() {
            return Err(format!("No {} found!", path));
        }

        let reader = File::open(path).expect(&format!("Failed to read {}", path));
        let json: Result<Map<String, Value>> = serde_json::from_reader(reader);
        if let Err(error) = json {
            return Err(format!("Failed parsing json: '{}'!", error.to_string()));
        }

        let data = json.unwrap();
        let script = data.get("scripts");
        if script.is_none() {
            return Err(format!("Key scripts not found!"));
        }

        let script = script.unwrap().as_object();
        if script.is_none() {
            return Err(format!(
                "Scripts key not an object: {:?}",
                data.get("scripts")
            ));
        }

        let mut data: HashMap<String, String> = HashMap::new();
        for (key, val) in script.unwrap().into_iter() {
            let val = val.as_str().unwrap().to_string();
            data.insert(key.to_string(), val);
        }

        Ok(Document {
            scripts: data,
            matcher: ClangdMatcher::default(),
        })
    }

    pub fn search(&self, query: &str) {
        for (k, v) in self.scripts.iter() {
            if let Some((_, indices)) = self.matcher.fuzzy_indices(&k, &query) {
                println!("{}: {}", Self::wrap_matches(&k, &indices), v);
            }
        }
    }

    fn wrap_matches(line: &str, indices: &[IndexType]) -> String {
        let mut ret = String::new();
        let mut peekable = indices.iter().peekable();
        for (idx, ch) in line.chars().enumerate() {
            let next_id = **peekable.peek().unwrap_or(&&(line.len() as IndexType));
            if next_id == (idx as IndexType) {
                ret.push_str(
                    format!(
                        "{}{}{}{}",
                        Bold,
                        termion::color::Fg(termion::color::Green),
                        ch,
                        Reset
                    )
                    .as_str(),
                );
                peekable.next();
            } else {
                ret.push(ch);
            }
        }

        ret
    }
}
