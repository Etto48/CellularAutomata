use crate::{NeuralCellularAutomata, Filter, DeepNeuralCellularAutomata};

pub fn new(size: (usize,usize)) -> DeepNeuralCellularAutomata
{
    DeepNeuralCellularAutomata::new(vec![
        NeuralCellularAutomata::new(
            Filter::new_gaussian(5, 2.0),
            include_str!("../shaders/activations/linear.wgsl"),
            include_str!("../shaders/color_filters/gray.wgsl"),
            size),
        NeuralCellularAutomata::new(
            Filter::new([
                0.0, -1.0, 0.0,
                -1.0, 4.0, -1.0,
                0.0, -1.0, 0.0
            ]),
            include_str!("../shaders/activations/linear.wgsl"),
            include_str!("../shaders/color_filters/gray.wgsl"),
            size),
        NeuralCellularAutomata::new(
            Filter::new_gaussian(5, 2.0),
            include_str!("../shaders/activations/linear.wgsl"),
            include_str!("../shaders/color_filters/gray.wgsl"),
            size),
        NeuralCellularAutomata::new(
            Filter::new([
                0.0, 1.65, -1.0,
                -1.0, 5.85, 1.65,
                1.65, -1.0, 0.0
            ]),
            include_str!("../shaders/activations/linear.wgsl"),
            include_str!("../shaders/color_filters/heatmap.wgsl"),
            size),
        
    ])
}
