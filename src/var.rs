use ndarray::Array1;
use std::ops::Add;

pub struct Var {
    sum_x: Array1<usize>,
    sum_x2: Array1<usize>,
    count: usize,
}

/* This class implemnts var calculation for leakage detection
Var = (sumx2/n) - ((sumx/n) ** 2)
*/
impl Var {
    pub fn new(size: usize) -> Self {
        Self {
            sum_x: Array1::zeros(size),
            sum_x2: Array1::zeros(size),
            count: 0,
        }
    }

    pub fn update(&mut self, trace: Array1<usize>) {
        for i in 0..self.sum_x.len() {
            self.sum_x[i] += trace[i];
            self.sum_x2[i] += trace[i] * trace[i];
        }
        self.count += 1;
    }

    pub fn finalize(&self) -> Array1<f32> {
        let len_result = self.sum_x.len();
        let mut result: Array1<f32> = Array1::zeros(len_result);
        for i in 0..len_result {
            let part_1 = self.sum_x2[i] as f32 / self.count as f32;
            let part_2: f32 = self.sum_x[i] as f32 / self.count as f32;
            result[i] = part_1 - part_2.powi(2);
        }
        result
    }
}

impl Add for Var {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            sum_x: self.sum_x + rhs.sum_x,
            sum_x2: self.sum_x2 + rhs.sum_x2,
            count: self.count + rhs.count,
        }
    }
}
