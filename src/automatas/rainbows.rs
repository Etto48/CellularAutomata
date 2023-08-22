use crate::{NeuralCellularAutomata, Filter};
pub fn new(size: (usize,usize)) -> NeuralCellularAutomata
{
    NeuralCellularAutomata::new(
        Filter::new([
            0.0, 2.0, 0.0,
            0.5, -2.0, 0.5,
            -0.5, 0.0, -0.5
        ]),
        include_str!("../shaders/activations/approximated_inverted_gaussian.wgsl"),
        include_str!("../shaders/color_filters/heatmap.wgsl"),
        size)
}