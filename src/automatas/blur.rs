use crate::{NeuralCellularAutomata, Filter};

pub fn new(size: (usize, usize)) -> NeuralCellularAutomata
{
    NeuralCellularAutomata::new(
        Filter::new_gaussian(3, 0.72),
        include_str!("../shaders/activations/linear.wgsl"),
        include_str!("../shaders/color_filters/gray.wgsl"),
        size)
}