use crate::{dnca::DeepNeuralCellularAutomata, nca::NeuralCellularAutomata};

pub fn new(size: (usize,usize)) -> DeepNeuralCellularAutomata {
    DeepNeuralCellularAutomata::new(vec![
        NeuralCellularAutomata::new(
            [
                [0.68, -0.9, 0.68],
                [-0.9, -0.66, -0.9],
                [0.68, -0.9, 0.68]
            ],
            include_str!("../shaders/activations/inverted_gaussian.wgsl"),
            include_str!("../shaders/color_filters/red.wgsl"),
            size),
        NeuralCellularAutomata::new(
            [
                [0.78, -0.9, 0.78],
                [-0.9, -0.96, -0.9],
                [0.78, -0.9, 0.78]
            ],
            include_str!("../shaders/activations/inverted_gaussian.wgsl"),
            include_str!("../shaders/color_filters/red.wgsl"),
            size),
        ])
    }