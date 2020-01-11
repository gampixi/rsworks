use sfml::graphics::{Font};
use sfml::system::{SfBox};
use std::collections::HashMap;

#[derive(Debug)]
pub enum LoadError {
    ShitsFuckedUp,
    NameTaken
}

#[derive(Debug)]
pub enum GetError {
    NameEmpty
}

pub struct ResourceManager {
    fonts: HashMap<String, SfBox<Font>>,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        eprintln!("Creating new ResourceManager...");
        ResourceManager {
            fonts: HashMap::new(),
        }
    }

    pub fn load_font(&mut self, filename: &str, fontname: &str) -> Result<(), LoadError> {
        if self.fonts.contains_key(fontname) {
            eprintln!("Font in {} already loaded (tried to load {})", fontname, filename);
            return Err(LoadError::NameTaken);
        }

        eprintln!("Loading {} as {}", filename, fontname);
        let new_font = Font::from_file(filename);
        match new_font {
            Some(font) => {
                self.fonts.insert(String::from(fontname), font);
                Ok(())
            }
            None => {
                Err(LoadError::ShitsFuckedUp)
            }
        }
    }

    pub fn get_font(&self, fontname: &str) -> Result<&SfBox<Font>, GetError> {
        let f = self.fonts.get(fontname);
        match f {
            Some(font) => Ok(font),
            None => Err(GetError::NameEmpty)
        }
    }
}