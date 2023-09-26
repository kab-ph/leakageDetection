use ndarray::{Array1, Array2};
use std::ops::Add;

pub struct ttest {
    sum_x: Array1<usize>,
    sum_x2: Array1<usize>,
    sum_y: Array1<usize>,
    sum_y2: Array1<usize>,
    count: usize,
}

/* This class implements a t-test between two sets share the same len  */

impl ttest {
    pub fn new(size: usize) -> Self {
        Self {
            sum_x: Array1::zeros(size),
            sum_x2: Array1::zeros(size),
            sum_y: Array1::zeros(size),
            sum_y2: Array1::zeros(size),
            count: 0,
        }
    }

    pub fn update(&mut self, trace_x: Array1<usize>, trace_y: Array1<usize>) {
        let len_trace = trace_x.len();
        for i in 0..len_trace {
            self.sum_x[i] += trace_x[i];
            self.sum_x2[i] += trace_x[i] * trace_x[i];
            self.sum_y[i] += trace_y[i];
            self.sum_y2[i] += trace_y[i] * trace_y[i];
        }
        self.count += 1;
    }

    pub fn finalize(&self) -> Array1<f64> {
        let len_result = self.sum_x.len();
        let mut result: Array1<f64> = Array1::zeros(len_result);
        for i in 0..len_result {
            let (var_x, var_y) = self.calculate_var(i);
            let numer: f64 = (self.sum_x[i] as f64 - self.sum_y[i] as f64) / self.count as f64;
            let denom = (var_x + var_y) as f64 / self.count as f64;
            //println!("{} {} {}", i, self.sum_x[i], self.sum_y[i]);
            result[i] = numer / f64::sqrt(denom);
        }
        result
    }

    pub fn calculate_var(&self, index: usize) -> (f64, f64) {
        let var_x: f64 = (self.sum_x2[index] as f64 / self.count as f64)
            - (self.sum_x[index] as f64 / self.count as f64).powi(2);
        let var_y: f64 = (self.sum_y2[index] as f64 / self.count as f64)
            - (self.sum_y[index] as f64 / self.count as f64).powi(2);
        (var_x, var_y)
    }
}

impl Add for ttest {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            sum_x: self.sum_x + rhs.sum_x,
            sum_x2: self.sum_x2 + rhs.sum_x2,
            sum_y: self.sum_y + rhs.sum_y,
            sum_y2: self.sum_y2 + rhs.sum_y2,
            count: self.count + rhs.count,
        }
    }
}
