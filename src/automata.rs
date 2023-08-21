use crate::ui::UI;

pub trait Automata {
    fn update(&mut self, ui: &mut UI);
}