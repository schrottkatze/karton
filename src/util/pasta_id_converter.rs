use std::fs;

use lazy_static::lazy_static;

use crate::args::ARGS;

const ANIMAL_NAMES: &[&str] = &[
    "ant", "eel", "mole", "sloth", "ape", "emu", "monkey", "snail", "bat", "falcon", "mouse",
    "snake", "bear", "fish", "otter", "spider", "bee", "fly", "parrot", "squid", "bird", "fox",
    "panda", "swan", "bison", "frog", "pig", "tiger", "camel", "gecko", "pigeon", "toad", "cat",
    "goat", "pony", "turkey", "cobra", "goose", "pug", "turtle", "crow", "hawk", "rabbit", "viper",
    "deer", "horse", "rat", "wasp", "dog", "jaguar", "raven", "whale", "dove", "koala", "seal",
    "wolf", "duck", "lion", "shark", "worm", "eagle", "lizard", "sheep", "zebra",
];

lazy_static!{
    pub static ref CONVERTER: PastaIdConverter = PastaIdConverter::new();
}

/// Convert pasta IDs to names and vice versa
pub struct PastaIdConverter {
    names: Vec<String>
}

impl PastaIdConverter {
    pub fn new() -> Self {
        let names;
        if let Some(names_path) = &ARGS.custom_names {
            let names_data = fs::read_to_string(names_path)
                .expect("path for the names file should contain a names file");
            names = names_data
                .split('\n')
                .map(ToOwned::to_owned)
                .collect::<Vec<String>>();
        } else {
            names = ANIMAL_NAMES
                .iter()
                .copied()
                .map(ToOwned::to_owned)
                .collect();
        }

        Self { names }
    }

    pub fn to_names(&self, mut number: u64) -> String {
        let mut result: Vec<&str> = Vec::new();

        if number == 0 {
            return self.names[0].parse().unwrap();
        }

        let mut power = 6;

        loop {
            let digit = number / self.names.len().pow(power) as u64;
            if !(result.is_empty() && digit == 0) {
                result.push(&self.names[digit as usize]);
            }
            number -= digit * self.names.len().pow(power) as u64;
            if power > 0 {
                power -= 1;
            } else if power == 0 || number == 0 {
                break;
            }
        }

        result.join("-")
    }

    pub fn to_u64(&self, pasta_id: &str) -> Result<u64, &str> {
        let mut result: u64 = 0;

        let names: Vec<&str> = pasta_id.split('-').collect();

        let mut pow = names.len();
        for name in names {
            pow -= 1;
            let name_index = self.names.iter().position(|r| r == name);
            match name_index {
                None => return Err("Failed to convert animal name to u64!"),
                Some(_) => {
                    result += (name_index.unwrap() * self.names.len().pow(pow as u32)) as u64
                }
            }
        }

        Ok(result)
    }
}

impl Default for PastaIdConverter {
    fn default() -> Self {
        Self::new()
    }
}


