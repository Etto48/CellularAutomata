use crate::{NeuralCellularAutomata, Filter};
pub fn new(size: (usize,usize)) -> NeuralCellularAutomata
{
    NeuralCellularAutomata::new(
        Filter::new([
            0.68, -0.9, 0.68,
            -0.9, -0.66, -0.9,
            0.68, -0.9, 0.68
        ]),
        include_str!("../shaders/activations/inverted_gaussian.wgsl"),
        include_str!("../shaders/color_filters/red.wgsl"),
        size)
}