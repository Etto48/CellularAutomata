use crate::{automata::Automata, ca::CellularAutomata};

#[derive(Debug)]
pub struct NeuralCellularAutomata
{
    _filter: [[f32;3];3],
    _activation_function: String,
    ca: CellularAutomata,
}

impl NeuralCellularAutomata
{
    pub fn new(filter: [[f32;3];3], activation_function: &str, color_filter: &str, size: (usize,usize)) -> Self { 
        //wgsl shader to apply filter to input texture and save to output texture
        //filter string must represent float with '.' even if they are integers
        let filter_string = format!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
            filter[0][0], filter[0][1], filter[0][2],
            filter[1][0], filter[1][1], filter[1][2],
            filter[2][0], filter[2][1], filter[2][2],
        );
        let reg = handlebars::Handlebars::new();
        let shader_file_text = include_str!("shaders/nca.wgsl");
        let shader_text = reg.render_template(
            shader_file_text, 
            &serde_json::json!({
                "filter": filter_string,
                "activation": activation_function,
                "color_filter": color_filter,
            })
        ).unwrap();

        Self { 
            _filter: filter, 
            _activation_function: activation_function.to_string(), 
            ca: CellularAutomata::new(&shader_text, size),
        }
    }
}

impl Automata for NeuralCellularAutomata
{
    fn update(&mut self, ui: &mut crate::ui::UI) {
        self.ca.update(ui);
    }
}