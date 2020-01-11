use sfml::graphics::{Transform, Drawable, RenderTarget};
use super::*;

pub trait Entity {
    fn info_mut(&mut self) -> &mut EntityInfo;
    fn info(&self) -> &EntityInfo;
    fn transform(&self) -> Option<&Transform> {
        None
    }
    fn draw(&self, target: &mut RenderTarget) {}
    fn safe_remove(&mut self) {
        self.info_mut().marked_for_removal = true;
        eprintln!("Entity {} marked for removal!", self.info().id);
    }

    fn init(&mut self) {}
    fn tick(&mut self) {}
}