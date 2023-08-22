use crate::Filter;

#[derive(Debug, Clone)]
pub enum Preset
{
    Conway,
    Worms,
    Worms2,
    Rainbows,
    Ghosts,
    Blur,
    Blur2,
    Cells,
    NCA{filter: Filter, activation: String, color_filter: String},
    DNCA{layers: Vec<(Filter,String,String)>},
    Custom{shader: String},
    Test,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipMode
{
    None,
    Even,
    Odd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Initialization
{
    Random,
    Zero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape
{
    Square,
    Circle,
}

#[derive(Debug, Clone, Copy)]
pub struct Pencil
{
    pub radius: usize,
    pub shape: Shape,
}


#[derive(Debug, Clone)]
pub struct Config
{
    pub size: (usize, usize),
    pub max_fps: Option<f32>,
    pub preset: Preset,
    pub skip_frames: SkipMode,
    pub initialization: Initialization,
    pub pencil: Pencil,
}

impl Config
{
    pub fn new() -> Self {
        Self {
            size: (800,600),
            max_fps: Some(60.0),
            preset: Preset::Test,
            skip_frames: SkipMode::None,
            initialization: Initialization::Zero,
            pencil: Pencil {
                radius: 1, shape: Shape::Circle},
        }
    }

    
    pub fn from_str(s: &str) -> Self {
        let mut ret = Self::new();
        let mut lines = s.lines();
        while let Some(line) = lines.next() {
            let mut parts = line.split('=');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            match key {
                "size" => {
                    let mut parts = value.split('x');
                    let x = parts.next().unwrap().parse().unwrap();
                    let y = parts.next().unwrap().parse().unwrap();
                    ret.size = (x,y);
                },
                "max_fps" => {
                    ret.max_fps = value.parse().ok();
                },
                "preset" => {
                    ret.preset = match value {
                        "conway" => Preset::Conway,
                        "worms" => Preset::Worms,
                        "worms2" => Preset::Worms2,
                        "rainbows" => Preset::Rainbows,
                        "ghosts" => Preset::Ghosts,
                        "blur" => Preset::Blur,
                        "test" => Preset::Test,
                        _ => {
                            let mut parts = value.split(',');
                            let filter = Filter::from_str(parts.next().unwrap());
                            let activation = parts.next().unwrap().to_string();
                            let color_filter = parts.next().unwrap().to_string();
                            Preset::NCA{filter, activation, color_filter}
                        },
                    }
                },
                "skip_frames" => {
                    ret.skip_frames = match value {
                        "none" => SkipMode::None,
                        "even" => SkipMode::Even,
                        "odd" => SkipMode::Odd,
                        _ => panic!("Invalid skip_frames value"),
                    }
                },
                "initialization" => {
                    ret.initialization = match value {
                        "random" => Initialization::Random,
                        "zero" => Initialization::Zero,
                        _ => panic!("Invalid initialization value"),
                    }
                },
                "pencil" => {
                    let mut parts = value.split(',');
                    let radius = parts.next().unwrap().parse().unwrap();
                    let shape = match parts.next().unwrap() {
                        "square" => Shape::Square,
                        "circle" => Shape::Circle,
                        _ => panic!("Invalid pencil shape"),
                    };
                    ret.pencil = Pencil{radius, shape};
                },
                _ => panic!("Invalid config key"),
            }
        }
        ret
    }
}