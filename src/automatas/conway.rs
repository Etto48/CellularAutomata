use crate::nca::NeuralCellularAutomata;

pub fn new(size: (usize,usize)) -> NeuralCellularAutomata
{
    NeuralCellularAutomata::new(
        [
            [1.0, 1.0, 1.0],
            [1.0, 9.0, 1.0],
            [1.0, 1.0, 1.0]
        ],
        include_str!("../shaders/activations/conway.wgsl"),
        include_str!("../shaders/color_filters/gray.wgsl"),
        size)
}