extern crate dirs;

use std::path::Path;
use std::fs;
use crate::matcher::Match;
use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    pub matches: Vec<Match>
}

impl Configs {
    pub fn load(path: &Path) -> Configs {
        let file_res = File::open(path);
        if let Ok(mut file) = file_res {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read config file");
            let config: Configs = serde_yaml::from_str(&contents)
                .expect("Unable to parse config file, invalid YAML syntax");

            config
        }else{
            panic!("Config file not found...")
        }
    }

    pub fn load_default() -> Configs {
        let res = dirs::home_dir();
        if let Some(home_dir) = res {
            let default_file = home_dir.join(".espanso");

            // If config file does not exist, create one from template
            if !default_file.exists() {
                fs::write(&default_file, DEFAULT_CONFIG_FILE_CONTENT)
                    .expect("Unable to write default config file");
            }

            Configs::load(default_file.as_path())
        }else{
            panic!("Could not generate default position for config file");
        }
    }
}

// TODO: add documentation link
const DEFAULT_CONFIG_FILE_CONTENT : &str = r###"# espanso configuration file
# This is the default configuration file, change it as you like it
# You can refer to the official documentation:

# Matches are the substitution rules, when you type the "trigger" string
# it gets replaced by the "replace" string.
matches:
  # Default
  - trigger: ":espanso"
    replace: "Hi there!"

  # Emojis
  - trigger: ":lol"
    replace: "😂"
  - trigger: ":llol"
    replace: "😂😂😂😂"
  - trigger: ":sad"
    replace: "☹"
  - trigger: ":ssad"
    replace: "☹☹☹☹"

  # Accented letters
  - trigger: "e'"
    replace: "è"
  - trigger: "e/"
    replace: "é"
  - trigger: "a'"
    replace: "à"
  - trigger: "i'"
    replace: "ì"
  - trigger: "o'"
    replace: "ò"
  - trigger: "u'"
    replace: "ù"

  # Capital accented letters
  - trigger: "E'"
    replace: "È"
  - trigger: "E/"
    replace: "É"
  - trigger: "A'"
    replace: "À"
  - trigger: "I'"
    replace: "Ì"
  - trigger: "O'"
    replace: "Ò"
  - trigger: "U'"
    replace: "Ù"
"###;