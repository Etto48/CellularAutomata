use cellular_automata::{
    ui::UI,
    automata::Automata,
    nca::NeuralCellularAutomata,
    config::{
        Config,
        Preset,
        SkipMode,
        Initialization, Pencil, Shape
    }, dnca::DeepNeuralCellularAutomata, automatas::{conway, worms, worms2, rainbows, ghosts, blur, test, blur2, cells}, ca::CellularAutomata};

fn main() {
    let config = Config{
        size: (800,600),
        max_fps: Some(120.0),
        preset: Preset::Cells,
        skip_frames: SkipMode::Odd,
        initialization: Initialization::Random,
        pencil: Pencil {
            radius: 6, shape: Shape::Circle},
    };

    let mut ui = match config.initialization {
        Initialization::Random => UI::new_random(config.size, config.pencil),
        Initialization::Zero => UI::new(config.size, config.pencil),
    };

    let mut nca: Box<dyn Automata> = match config.preset {
        Preset::Conway => Box::new(conway::new(ui.get_size())),
        Preset::Worms => Box::new(worms::new(ui.get_size())),
        Preset::Worms2 => Box::new(worms2::new(ui.get_size())),
        Preset::Rainbows => Box::new(rainbows::new(ui.get_size())),
        Preset::Ghosts => Box::new(ghosts::new(ui.get_size())),
        Preset::Blur => Box::new(blur::new(ui.get_size())),
        Preset::Blur2 => Box::new(blur2::new(ui.get_size())),
        Preset::Cells => Box::new(cells::new(ui.get_size())),
        Preset::NCA{filter, activation, color_filter} => Box::new(NeuralCellularAutomata::new(filter, activation.as_str(), color_filter.as_str(), ui.get_size())),
        Preset::DNCA{layers} => Box::new(DeepNeuralCellularAutomata::new(layers.into_iter().map(|(filter, activation, color_filter)| NeuralCellularAutomata::new(filter, activation.as_str(), color_filter.as_str(), ui.get_size())).collect())),
        Preset::Custom{shader}=> Box::new(CellularAutomata::new(shader.as_str(),ui.get_size())),
        Preset::Test => Box::new(test::new(ui.get_size())),
    };

    //let mut nca = cells::new(ui.get_size());

    let skip_frames = config.skip_frames != SkipMode::None;
    let mut skip_this_frame = match config.skip_frames {
        SkipMode::None => false,
        SkipMode::Even => true,
        SkipMode::Odd => false,
    };

    let max_fps = config.max_fps.unwrap_or(0.0);
    let ms_per_frame = 1000.0/max_fps;
    ui.update();
    while ui.get_window().is_open() && !ui.get_window().is_key_down(minifb::Key::Escape) {
        let start = std::time::Instant::now();
        //apply nca to ui buffer
        nca.update(&mut ui);
        if skip_this_frame  && skip_frames{
            skip_this_frame = false;
            continue;
        }
        else {
            skip_this_frame = true;
        }
        //draw ui buffer to screen
        ui.update();
        let elapsed = start.elapsed().as_millis() as f32;
        if max_fps != 0.0 && elapsed < ms_per_frame {
            std::thread::sleep(std::time::Duration::from_millis((ms_per_frame-elapsed) as u64));
        }
    }

}
