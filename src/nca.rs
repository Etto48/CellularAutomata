use crate::{automata::Automata, ui::UI};

pub struct NeuralCellularAutomata
{
    filter: [[f32;3];3],
    activation_function: fn(f32) -> f32,
}

impl NeuralCellularAutomata
{
    pub fn new(filter: [[f32;3];3], activation_function: fn(f32)->f32) -> Self { 
        Self { filter , activation_function }
    }
}

impl Automata for NeuralCellularAutomata
{
    fn update(&mut self, ui: &mut UI) {
        let size = ui.get_size();
        let mut new_buffer = vec![0.0;size.0 * size.1];
        for x in 0..size.0 {
            for y in 0..size.1 {
                let mut sum = 0.0;
                for i in 0..3 {
                    for j in 0..3 {
                        let x = (x + i - 1) % size.0;
                        let y = (y + j - 1) % size.1;
                        sum += *ui.pixel(x, y) * self.filter[2-j][i];
                    }
                }
                new_buffer[x + y * size.0] = (self.activation_function)(sum);

            }
        }
        ui.buffer = new_buffer;
    }
}