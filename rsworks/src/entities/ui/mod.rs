use crate::entities::traits::Entity;
use crate::entities::EntityInfo;
use sfml::system::{SfBox};
use sfml::graphics::{Font, Text, Drawable, RenderTarget, Color};

pub struct UIText {
    info: EntityInfo,
    font: SfBox<Font>,
    pub text: String,
    pub size: u32,
}

impl Entity for UIText {
    fn info_mut(&mut self) -> &mut EntityInfo {
        &mut self.info
    }

    fn info(&self) -> &EntityInfo {
        &self.info
    }

    fn init(&mut self) {
        //self.text = String::from("");
    }

    fn draw(&self, target: &mut RenderTarget) {
        let mut t = Text::new(&self.text, &self.font, self.size);
        t.set_fill_color(Color::WHITE);
        target.draw(&t);
    }
}

impl UIText {
    pub fn new(font: &SfBox<Font>) -> UIText {
        UIText {
            info: EntityInfo {id: 0, marked_for_removal: false},
            text: String::from(""),
            size: 18,
            font: font.clone()
        }
    }

    pub fn set_font(&mut self, font: &SfBox<Font>) {
        self.font.clone_from(font);
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }
}