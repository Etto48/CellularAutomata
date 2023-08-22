use crate::{nca::NeuralCellularAutomata, automata::Automata, ui::UI};

#[derive(Debug)]
pub struct DeepNeuralCellularAutomata
{
    layers: Vec<NeuralCellularAutomata>,
}

impl DeepNeuralCellularAutomata
{
    pub fn new(layers: Vec<NeuralCellularAutomata>) -> Self {
        Self {
            layers,
        }
    }
}

impl Automata for DeepNeuralCellularAutomata
{
    fn update(&mut self, ui: &mut UI) {
        for layer in &mut self.layers {
            layer.update(ui);
        }
    }
}

