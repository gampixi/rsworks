use sfml::graphics::{Color, Font, RenderWindow, RenderTarget, RenderStates, Text};
use sfml::window;
use sfml::window::{Event, Key};
use sfml::system::{Clock, Time, SfBox};
use std::process;
use std::error::Error;

pub mod resources;
use resources::ResourceManager;

pub mod entities;
use entities::EntityManager;

pub struct Game {
    pub render_window: RenderWindow,
    pub events: Vec<Event>,
    pub resources: ResourceManager,
    pub entities: EntityManager,
}

impl Game {
    pub fn new(title: &str) -> Game {
        eprintln!("Initializing game {}...", title);
        Game {
            events: vec![],
            resources: ResourceManager::new(),
            entities: EntityManager::new(),
            render_window: create_window(title),
        }
    }

    pub fn begin_cycle(&mut self) {
        let ml = self.main_loop().expect("Shit.");
    }

    pub fn end_cycle(&mut self) {
        let rl = self.render_loop().expect("Double shit.");
    }

    fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        self.events = self.read_events();
        self.entities.perform_tick();
        self.entities.remove_marked();
        Ok(())
    }
    
    fn render_loop(&mut self) -> Result<(), Box<dyn Error>> {
        self.render_window.clear(Color::BLACK);
        self.entities.perform_draw(&mut self.render_window);
        self.render_window.display();
        Ok(())
    }
    
    fn read_events(&mut self) -> Vec<Event> {
        // Event read loop
        let mut events: Vec<Event> = vec!();
        loop {
            if let Some(e) = self.render_window.poll_event() {
                events.push(e);
            }
            else {
                return events;
            }
        }
    }
}

fn create_window(title: &str) -> RenderWindow {
    let desktop = window::VideoMode::desktop_mode();
    let video_mode = window::VideoMode::new(640, 480, desktop.bits_per_pixel);
    let style = window::Style::TITLEBAR;
    let context_settings = window::ContextSettings {
        depth_bits: 8,
        stencil_bits: 8,
        antialiasing_level: 1,
        major_version: 3,
        minor_version: 2,
        attribute_flags: window::ContextSettings::ATTRIB_DEFAULT,
        srgb_capable: 1,
    };

    RenderWindow::new(video_mode, title, style, &context_settings)
}
