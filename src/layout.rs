use serde::Deserialize;

use crate::config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyFamily {
    Normal,
    Shift,
}

#[derive(Deserialize)]
pub struct Layout {
    normal: Vec<char>,
    shift: Vec<char>,
}

impl Layout {
    pub fn new(layout_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let layout_address =
            Self::parse_layout_name(layout_name).unwrap_or_else(|_| layout_name.to_string());

        let content = std::fs::read_to_string(layout_address)
            .map_err(|_| format!("Layout {layout_name} not found."))?;

        let layout = serde_json::from_str(&content)?;

        Ok(layout)
    }

    fn parse_layout_name(layout_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let (language, variant) = layout_name.split_once(':').ok_or("Invalid layout name.")?;

        Ok(format!(
            "{}/{}/{}.json",
            config::get_layout_dir(),
            language,
            variant
        ))
    }

    pub fn get_character_index(&self, character: char) -> Option<(KeyFamily, usize)> {
        if let Some(index) = self.normal.iter().position(|&ch| ch == character) {
            return Some((KeyFamily::Normal, index));
        }

        if let Some(index) = self.shift.iter().position(|&ch| ch == character) {
            return Some((KeyFamily::Shift, index));
        }

        None
    }

    pub fn get_index_character(&self, family: KeyFamily, index: usize) -> Option<char> {
        match family {
            KeyFamily::Normal => self.normal.get(index).copied(),
            KeyFamily::Shift => self.shift.get(index).copied(),
        }
    }
}

fn map_character(character: char, source_layout: &Layout, target_layout: &Layout) -> char {
    let Some((family, index)) = source_layout.get_character_index(character) else {
        return character;
    };
    target_layout
        .get_index_character(family, index)
        .unwrap_or(character)
}

pub fn map_string(string: &str, source_layout: &Layout, target_layout: &Layout) -> String {
    string
        .chars()
        .map(|character| map_character(character, source_layout, target_layout))
        .collect()
}
