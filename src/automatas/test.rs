use crate::nca::NeuralCellularAutomata;
pub fn new(size: (usize,usize)) -> NeuralCellularAutomata
{
    NeuralCellularAutomata::new(
        [
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0]
        ],
        include_str!("../shaders/activations/linear.wgsl"),
        include_str!("../shaders/color_filters/gray.wgsl"),
        size)
}