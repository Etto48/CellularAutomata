use minifb::{Window, WindowOptions};

pub struct UI {
    window: Window,
    pub buffer: Vec<u32>,
    size: (usize, usize),
}

impl UI
{
    pub fn new(size: (usize, usize)) -> Self {
        let window = Window::new(
            "NeuralCellularAutomata - ESC to exit",
            size.0,
            size.1,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let buffer = vec![0;size.1 * size.0];

        Self {
            window,
            buffer,
            size,
        }
    }

    pub fn pixel(&mut self,x: usize, y: usize) -> &mut u32
    {
        &mut self.buffer[x+y*self.size.0]
    }

    pub fn new_random(size: (usize, usize)) -> Self {
        let mut ret = Self::new(size);
        for x in 0..size.0 {
            for y in 0..size.1 {
                *ret.pixel(x, y) = rand::random::<u32>();
            }
        }
        ret
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.size
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.size.0, self.size.1)
            .unwrap();
    }
}