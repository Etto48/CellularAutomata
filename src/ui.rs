use minifb::{Window, WindowOptions};

pub struct UI {
    window: Window,
    colors: Vec<(f32,f32,f32)>,
    pub buffer: Vec<f32>,
    size: (usize, usize),
}

impl UI
{
    pub fn new(size: (usize, usize), style: &str) -> Self {
        let window = Window::new(
            "Test - ESC to exit",
            size.0,
            size.1,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let buffer = vec![0.0;size.1 * size.0];

        Self {
            window,
            buffer,
            colors: match style {
                "full_range" => vec![
                        (0.0,0.0,0.0),
                        (1.0,0.0,0.0),
                        (0.0,1.0,0.0),
                        (0.0,0.0,1.0),
                        (1.0,1.0,1.0)],
                "red" => vec![
                        (0.0,0.0,0.0),
                        (1.0,0.0,0.0)],
                "green" => vec![
                        (0.0,0.0,0.0),
                        (0.0,1.0,0.0)],
                "blue" => vec![
                    (0.0,0.0,0.0),
                    (0.0,0.0,1.0)],
                "grey" => vec![
                    (0.0,0.0,0.0),
                    (1.0,1.0,1.0)],
                _ => vec![
                    (0.0,0.0,0.0),
                    (1.0,1.0,1.0)]
            },
            size,
        }
    }

    pub fn pixel(&mut self,x: usize, y: usize) -> &mut f32
    {
        &mut self.buffer[x+y*self.size.0]
    }

    pub fn new_random(size: (usize, usize),min: f32, max: f32, style: &str) -> Self {
        let mut ret = Self::new(size,style);
        for x in 0..size.0 {
            for y in 0..size.1 {
                *ret.pixel(x, y) = rand::random::<f32>() * (max - min) + min;
            }
        }
        ret
    }

    fn f32_to_u8(f: f32) -> u8 {
        (f.clamp(0.0, 1.0) * 255.0) as u8
    }

    fn from_f32_rgb(r: f32, g: f32, b: f32) -> u32 {
        let (r, g, b) = (Self::f32_to_u8(r), Self::f32_to_u8(g), Self::f32_to_u8(b));
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    fn color_lerp(color1: (f32,f32,f32), color2: (f32,f32,f32), t: f32) -> (f32,f32,f32)
    {
        let (r1,g1,b1) = color1;
        let (r2,g2,b2) = color2;
        let r = r1 + (r2 - r1) * t;
        let g = g1 + (g2 - g1) * t;
        let b = b1 + (b2 - b1) * t;
        (r,g,b)
    }

    fn from_f32_x(&self,x: f32) -> u32
    {
        //lerp between colors based on x
        let intervals = self.colors.len() - 1;
        let value = x.clamp(0.0, 1.0);
        let interval = (value * intervals as f32).floor() as usize;
        let t = value * intervals as f32 - interval as f32;
        let color = if interval == intervals
        {
            self.colors[interval]
        }
        else {
            Self::color_lerp(self.colors[interval], self.colors[interval+1], t)
        };

        Self::from_f32_rgb(color.0, color.1, color.2)
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.size
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn update(&mut self) {
        let mut buffer = vec![0;self.size.0 * self.size.1];
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let index = y * self.size.0 + x;
                let x = *self.pixel(x, y);
                buffer[index] = self.from_f32_x(x);
            }
        }
        self.window
            .update_with_buffer(&buffer, self.size.0, self.size.1)
            .unwrap();
    }
}