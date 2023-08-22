use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Filter
{
    pub data: Vec<f32>,
    size: usize,
}

impl Filter
{
    pub fn new<const N: usize>(data: [f32;N]) -> Self {
        let possible_values = [1*1, 3*3, 5*5, 7*7, 9*9];
        assert!(possible_values.contains(&N), "Filter data must be 3x3, 5x5, 7x7, or 9x9");
        let size = (N as f32).sqrt() as usize;
        Self { data: data.to_vec() , size }
    }

    pub fn new_identity() -> Self {
        Self::new([1.0])
    }

    pub fn new_gaussian(size: usize, sigma: f32) -> Self {
        let mut data = vec![0.0; size*size];
        let center = size/2;
        let sigma2 = sigma*sigma;
        let mut sum = 0.0;
        for i in 0..size {
            for j in 0..size {
                let x = i as f32 - center as f32;
                let y = j as f32 - center as f32;
                let value = (-(x*x + y*y)/(2.0*sigma2)).exp();
                data[i*size + j] = value;
                sum += value;
            }
        }
        for i in 0..size {
            for j in 0..size {
                data[i*size + j] /= sum;
            }
        }
        Self { data, size }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn from_str(_s: &str) -> Self {
        todo!()
    }
}

impl Display for Filter
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data_string = String::new();
        for i in 0..self.data.len() {
            data_string.push_str(&format!("{:?}",self.data[i]));
            if i != self.data.len()-1 {
                data_string.push(',');
            }
        }
        write!(f, "{}", data_string)
    }
}