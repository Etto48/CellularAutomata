use cellular_automata::{ui::UI, nca::NeuralCellularAutomata, automata::Automata, nca_gpu::NeuralCellularAutomataGPU};

fn main() {
    let mut ui = UI::new_random((800,600),-1.0,1.0,"grey");
    let _conway = NeuralCellularAutomata::new(
        [
            [1.0, 1.0, 1.0],
            [1.0, 9.0, 1.0],
            [1.0, 1.0, 1.0]
        ],
        |x| {
            if x < 0.0
            {
                1.0
            }
            else if x == 3.0 || x == 11.0 || x == 12.0 {
                1.0
            }
            else {
                0.0
            }
        },
    );

    let _worms = NeuralCellularAutomata::new(
        [
            [0.68, -0.9, 0.68],
            [-0.9, -0.66, -0.9],
            [0.68, -0.9, 0.68]
        ],
        |x| {
            //-1./2.0_f32.powf(0.6*x.powi(2))+1.0
            let x2 = x*x;
            let x4 = x2*x2;
            let x8 = x4*x4;
            1.0-(1.0+35.1*x2+3.5*x4)/(1.0+35.1*x2+20.7*x4+2.3*x8)
        },
    );

    let _triangles = NeuralCellularAutomata::new(
        [
            [0.0, 2.0, 0.0],
            [0.5, -2.0, 0.5],
            [-0.5, 0.0, -0.5]
        ],
        |x| {
            let x2 = x*x;
            let x4 = x2*x2;
            let x8 = x4*x4;
            1.0-(1.0+30.1*x2+2.0*x4)/(1.0+35.1*x2+20.7*x4+1.0*x8)
        },
    );

    let _triangles_gpu = NeuralCellularAutomataGPU::new(
        [
            [0.0, 2.0, 0.0],
            [0.5, -2.0, 0.5],
            [-0.5, 0.0, -0.5]
        ],
        include_str!("shaders/approximated_inverted_gaussian.wgsl"),
        ui.get_size());
    
    let _worms_gpu = NeuralCellularAutomataGPU::new(
        [
            [0.68, -0.9, 0.68],
            [-0.9, -0.66, -0.9],
            [0.68, -0.9, 0.68]
        ],
        include_str!("shaders/inverted_gaussian.wgsl"),
        ui.get_size());

    let _conway_gpu = NeuralCellularAutomataGPU::new(
        [
            [1.0, 1.0, 1.0],
            [1.0, 9.0, 1.0],
            [1.0, 1.0, 1.0]
        ],
        include_str!("shaders/conway.wgsl"),
        ui.get_size());

    let _test_gpu = NeuralCellularAutomataGPU::new(
        [
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0]
        ],
        include_str!("shaders/test_activation.wgsl"),
        ui.get_size());

    let mut nca = _worms_gpu;

    let mut skip_this_frame = false;
    let target_fps = 60.0;
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
        if elapsed < ms_per_frame {
            std::thread::sleep(std::time::Duration::from_millis((ms_per_frame-elapsed) as u64));
        }
    }

}
