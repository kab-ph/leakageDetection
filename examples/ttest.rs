use indicatif::ProgressIterator;
use leakageDetection::tools::{progress_bar, read_array_2_from_npy_file, save_array};
use leakageDetection::ttest::ttest;
use ndarray::Array2;
use rayon::prelude::{ParallelBridge, ParallelIterator};

fn t_test() {
    let size = 5000;
    type formatTraces = i16;
    let n_files = 5;
    let folder = String::from("../data_ttest");

    let tt = (0..n_files)
        .progress_with(progress_bar(n_files))
        .into_iter()
        .map(|n| {
            let dir_x = format!("{folder}/fixed/l{n}.npy");
            let dir_y = format!("{folder}/variable/l{n}.npy");
            let leakage_x: Array2<formatTraces> = read_array_2_from_npy_file(&dir_x);
            let leakage_y: Array2<formatTraces> = read_array_2_from_npy_file(&dir_y);
            (leakage_x, leakage_y)
        })
        .into_iter()
        .par_bridge()
        .map(|patch| {
            let mut t = ttest::new(size);
            for i in 0..patch.0.shape()[0] {
                t.update(
                    patch.0.row(i).map(|x| *x as usize),
                    patch.1.row(i).map(|y| *y as usize),
                );
            }
            t
        })
        .reduce(|| ttest::new(size), |a, b| a + b);

    save_array("../results/ttest.npy", tt.finalize());
}

fn main() {
    t_test();
}
