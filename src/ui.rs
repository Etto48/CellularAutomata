use minifb::{Window, WindowOptions};

use crate::config::{Pencil, Shape};

pub struct UI {
    window: Window,
    pub buffer: Vec<u32>,
    size: (usize, usize),
    pencil: Pencil,
}

impl UI
{
    pub fn new(size: (usize, usize), pencil: Pencil) -> Self {
        let window = Window::new(
            "NeuralCellularAutomata - ESC to exit",
            size.0,
            size.1,
            WindowOptions{
                borderless: false,
                title: true,
                resize: false,
                scale: minifb::Scale::X1,
                scale_mode: minifb::ScaleMode::AspectRatioStretch,
                topmost: false,
                transparency: false,
                none: false,
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let buffer = vec![0;size.1 * size.0];

        Self {
            window,
            buffer,
            size,
            pencil,
        }
    }

    pub fn pixel(&mut self,x: usize, y: usize) -> &mut u32
    {
        &mut self.buffer[x+y*self.size.0]
    }

    pub fn new_random(size: (usize, usize), pencil: Pencil) -> Self {
        let mut ret = Self::new(size, pencil);
        for x in 0..size.0 {
            for y in 0..size.1 {
                let b: u8 = rand::random();
                let value: u32 = (b as u32) << 24 | (b as u32) << 16 | (b as u32) << 8 | (b as u32);
                *ret.pixel(x, y) = value;
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
        let lmb = self.window.get_mouse_down(minifb::MouseButton::Left);
        let rmb = self.window.get_mouse_down(minifb::MouseButton::Right);

        if lmb || rmb
        {
            if let Some((x, y)) = self.window.get_mouse_pos(minifb::MouseMode::Discard)
            {
                let x = x as usize;
                let y = y as usize;
                if x < self.size.0 && y < self.size.1 {
                    //set each pixel in a 3x3 square to white
                    let l = self.pencil.radius as i32;
                    let c: u32 = if lmb { 0xFFFFFFFF } else { 0x00000000 };
                    for dx in -l..=l {
                        for dy in -l..=l {
                            if self.pencil.shape == Shape::Circle && dx*dx + dy*dy > l*l { continue; }
                            let x = ((dx + (x + self.size.0) as i32) % self.size.0 as i32) as usize;
                            let y = ((dy + (y + self.size.1) as i32) % self.size.1 as i32) as usize;
                            *self.pixel(x, y) = c;
                        }
                    }
                    
                }
            }
        }
        self.window
            .update_with_buffer(&self.buffer, self.size.0, self.size.1)
            .unwrap();
    }
}