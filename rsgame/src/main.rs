use rsworks::Game;
use rsworks::entities::ui::UIText;

fn main() {
    let mut g = Game::new("RsWorks Game");

    load_resources(&mut g);
    create_text_entity(&mut g);

    loop {
        g.begin_cycle();
        game_loop(&mut g);
        g.end_cycle();
    }
}

fn load_resources(g: &mut Game) {
    eprintln!("Loading all resources...");
    g.resources.load_font("fonts/ROBOTO-REGULAR.ttf", "Roboto Regular")
        .expect("Oh shit oh fuck");
}

fn create_text_entity(g: &mut Game) {
    let font = g.resources.get_font("Roboto Regular").expect("Gah!");
    let mut ent = UIText::new(&font);
    ent.set_size(24);
    ent.set_text("Howdy, partner!");
    g.entities.add(Box::new(ent));
}

fn game_loop(g: &mut Game) {

}