use sfml::graphics::RenderTarget;
pub mod traits;
use traits::*;
pub mod ui;
use ui::*;

pub struct EntityInfo {
    pub id: u64,
    marked_for_removal: bool,
}

impl EntityInfo {
    fn copy(&self, other: &mut EntityInfo) {
        other.id = self.id;
        other.marked_for_removal = self.marked_for_removal;
    }
}

pub struct EntityManager {
    current_id: u64,
    entities: Vec<Box<dyn Entity>>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        eprintln!("Creating new EntityManager...");
        EntityManager {
            current_id: 0,
            entities: vec!(),
        }
    }

    fn get_new_id(&mut self) -> u64 {
        self.current_id += 1;
        self.current_id
    }

    pub fn add(&mut self, ent: Box<dyn Entity>) {
        let mut new_ent = ent;
        let info = EntityInfo {
            id: self.get_new_id(),
            marked_for_removal: false
        };
        info.copy(new_ent.info_mut());
        new_ent.init();
        self.entities.push(new_ent);
    }

    pub fn perform_tick(&mut self) {
        for i in self.entities.iter_mut() {
            i.tick();
        }
    }

    pub fn perform_draw(&mut self, target: &mut RenderTarget) {
        for i in self.entities.iter_mut() {
            i.draw(target);
        }
    }

    //remove assumes that any cleanup necessary by Entity has been done
    pub fn remove_marked(&mut self) {
        let mut remove_idx: Vec<usize> = vec!();

        for (i, ent) in self.entities.iter().enumerate() {
            if ent.info().marked_for_removal {
                remove_idx.push(i);
                eprintln!("Entity {} ready to remove!", ent.info().id);
            }
        }

        remove_idx.sort_unstable();
        remove_idx.reverse();

        for i in remove_idx {
            self.entities.remove(i);
        }
    }
}