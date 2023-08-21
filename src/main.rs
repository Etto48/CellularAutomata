use cellular_automata::{ui::UI, /*nca::NeuralCellularAutomata,*/ automata::Automata, nca::NeuralCellularAutomata};

fn main() {
    let mut ui = UI::new_random((800,600));

    let _triangles_gpu = NeuralCellularAutomata::new(
        [
            [0.0, 2.0, 0.0],
            [0.5, -2.0, 0.5],
            [-0.5, 0.0, -0.5]
        ],
        include_str!("shaders/activations/approximated_inverted_gaussian.wgsl"),
        include_str!("shaders/color_filters/heatmap.wgsl"),
        ui.get_size());
    
    let _worms_gpu = NeuralCellularAutomata::new(
        [
            [0.68, -0.9, 0.68],
            [-0.9, -0.66, -0.9],
            [0.68, -0.9, 0.68]
        ],
        include_str!("shaders/activations/inverted_gaussian.wgsl"),
        include_str!("shaders/color_filters/heatmap.wgsl"),
        ui.get_size());

    let _conway_gpu = NeuralCellularAutomata::new(
        [
            [1.0, 1.0, 1.0],
            [1.0, 9.0, 1.0],
            [1.0, 1.0, 1.0]
        ],
        include_str!("shaders/activations/conway.wgsl"),
        include_str!("shaders/color_filters/gray.wgsl"),
        ui.get_size());

    let _test_gpu = NeuralCellularAutomata::new(
        [
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0]
        ],
        include_str!("shaders/activations/test.wgsl"),
        include_str!("shaders/color_filters/gray.wgsl"),
        ui.get_size());

    let mut nca = _triangles_gpu;

    let mut skip_this_frame = false;
    let target_fps = 120.0;
    let ms_per_frame = 1000.0/target_fps;
    while ui.get_window().is_open() && !ui.get_window().is_key_down(minifb::Key::Escape) {
        let start = std::time::Instant::now();
        nca.update(&mut ui);
        if skip_this_frame {
            skip_this_frame = false;
            continue;
        }
        else {
            skip_this_frame = true;
        }
        ui.update();
        let elapsed = start.elapsed().as_millis() as f32;
        if target_fps != 0.0 && elapsed < ms_per_frame {
            std::thread::sleep(std::time::Duration::from_millis((ms_per_frame-elapsed) as u64));
        }
    }

}
