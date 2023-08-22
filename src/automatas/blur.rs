use crate::nca::NeuralCellularAutomata;

pub fn new(size: (usize, usize)) -> NeuralCellularAutomata
{
    NeuralCellularAutomata::new(
        [
            [1.0/16.0,1.0/8.0,1.0/16.0],
            [1.0/8.0,1.0/4.0,1.0/8.0],
            [1.0/16.0,1.0/8.0,1.0/16.0]
        ],
        include_str!("../shaders/activations/linear.wgsl"),
        include_str!("../shaders/color_filters/gray.wgsl"),
        size)
}