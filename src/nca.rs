use crate::{Filter, CellularAutomata, Automata};


#[derive(Debug)]
pub struct NeuralCellularAutomata
{
    _filter: Filter,
    _activation_function: String,
    ca: CellularAutomata,
}

impl NeuralCellularAutomata
{
    pub fn new(filter: Filter, activation_function: &str, color_filter: &str, size: (usize,usize)) -> Self { 
        //wgsl shader to apply filter to input texture and save to output texture
        //filter string must represent float with '.' even if they are integers
        let reg = handlebars::Handlebars::new();
        let shader_file_text = include_str!("shaders/nca.wgsl");
        let shader_text = reg.render_template(
            shader_file_text, 
            &serde_json::json!({
                "filter": filter.to_string(),
                "filter_size": filter.get_size(),
                "filter_data_len": filter.data.len(),
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