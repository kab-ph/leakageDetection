use indicatif::ProgressIterator;
use leakageDetection::tools::{progress_bar, read_array_2_from_npy_file, save_array};
use leakageDetection::var::Var;
use ndarray::Array2;
use rayon::prelude::{ParallelBridge, ParallelIterator};

fn var() {
    let size = 5000;
    type formatTraces = i16;
    let n_files = 5;
    let folder = String::from("../data");

    let var = (0..n_files)
        .progress_with(progress_bar(n_files))
        .into_iter()
        .map(|n| {
            let dir = format!("{folder}/l{n}.npy");
            let leakage: Array2<formatTraces> = read_array_2_from_npy_file(&dir);
            leakage
        })
        .into_iter()
        .par_bridge()
        .map(|patch| {
            let mut v = Var::new(size);
            for i in 0..patch.shape()[0] {
                v.update(patch.row(i).map(|sample| *sample as usize));
            }
            v
        })
        .reduce(|| Var::new(size), |a, b| a + b);

    //save result
    save_array("../results/var.npy", var.finalize());
}

fn main() {
    var();
}
