use dirs;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod syntax_highlight;

use self::syntax_highlight::SyntaxHighlight;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub visible_line_number: bool,
    pub syntax_highlight: SyntaxHighlight,
}

impl Config {
    pub fn load() -> Self {
        let mut pathbuf = dirs::home_dir().expect("internal error: can not find home directory.");
        pathbuf.push(".config/ysd/config.toml");
        Self::from_file(pathbuf.as_path()).unwrap_or_else(|e| {
            eprintln!("invalid config file: {}", e);
            Config::default()
        })
    }

    // TODO: use better error type, not String
    pub fn from_file(filename: &Path) -> Result<Self, String> {
        let mut config_file = String::new();
        File::open(filename)
            .and_then(|mut f| f.read_to_string(&mut config_file))
            .expect("can not open file");

        toml::from_str(config_file.as_str()).map_err(|e| format!("{}", e))
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            visible_line_number: false,
            syntax_highlight: SyntaxHighlight::default(),
        }
    }
}
