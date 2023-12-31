use crate::{NeuralCellularAutomata, Filter, DeepNeuralCellularAutomata};
pub fn new(size: (usize,usize)) -> DeepNeuralCellularAutomata
{
    DeepNeuralCellularAutomata::new(vec![
        NeuralCellularAutomata::new(
            Filter::new([
                0.5, 1.0, -2.0,
                1.0, -2.0, 1.0,
                -1.0, 1.0, 0.5
            ]),
            include_str!("../shaders/activations/linear.wgsl"),
            include_str!("../shaders/color_filters/gray.wgsl"),
            size),
        NeuralCellularAutomata::new(
            Filter::new([
                -0.5, -1.0, 1.0,
                -1.0, -1.5, -1.0,
                2.0, -1.0, -0.5
            ]),
            include_str!("../shaders/activations/linear.wgsl"),
            include_str!("../shaders/color_filters/gray.wgsl"),
            size),
        NeuralCellularAutomata::new(
            Filter::new([
                1.0/16.0,1.0/8.0,1.0/16.0,
                1.0/8.0,1.0/4.0,1.0/8.0,
                1.0/16.0,1.0/8.0,1.0/16.0
            ]),
            include_str!("../shaders/activations/linear.wgsl"),
            include_str!("../shaders/color_filters/gray.wgsl"),
            size),
    ])
}
