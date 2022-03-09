use serde_json::{Map, Result, Value};
use std::collections::HashMap;
use std::result::Result as ResultError;
use std::{fs::File, path::Path};

use fuzzy_matcher::clangd::ClangdMatcher;

use fuzzy_matcher::FuzzyMatcher;

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

        let reader = File::open(path).unwrap_or_else(|_| panic!("Failed to read {}", path));
        let json: Result<Map<String, Value>> = serde_json::from_reader(reader);
        if let Err(error) = json {
            return Err(format!("Failed parsing json: '{}'!", error));
        }

        let data = json.unwrap();
        let script = data.get("scripts");
        if script.is_none() {
            return Err(String::from("Key scripts not found!"));
        }

        let script = script.unwrap().as_object();
        if script.is_none() {
            return Err(format!(
                "Scripts key not an object: {:?}",
                data.get("scripts")
            ));
        }

        let mut scripts: HashMap<String, String> = HashMap::new();
        for (key, val) in script.unwrap() {
            let val = val.as_str().unwrap().to_string();
            scripts.insert(format!("{} : {}", key, val.to_string()), key.to_string());
        }

        Ok(Document {
            scripts,
            matcher: ClangdMatcher::default(),
        })
    }

    pub fn get_script(&self, key: &str) -> Option<&String> {
        self.scripts.get(key)
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        let mut data = vec![];
        for (k, _) in &self.scripts {
            if let Some((_, _)) = self.matcher.fuzzy_indices(k, query) {
                data.push(k.to_string());
            }
        }
        data
    }
}
