use indicatif::{ProgressBar, ProgressStyle};
use ndarray::{Array, Array2, ArrayView2};
use ndarray_npy::write_npy;
use ndarray_npy::{ReadNpyExt, ReadableElement, WriteNpyExt};
use std::io::BufWriter;
use std::{fs::File, time::Duration};

pub fn read_array_2_from_npy_file<T: ReadableElement>(dir: &str) -> Array2<T> {
    let reader: File = File::open(dir).unwrap();
    let arr: Array2<T> = Array2::<T>::read_npy(reader).unwrap();
    arr
}

pub fn save_array<T: ReadableElement + ndarray_npy::WritableElement, D: ndarray::Dimension>(
    path: &str,
    arr: Array<T, D>,
) {
    write_npy(path, &arr).unwrap();
}

pub fn progress_bar(len: usize) -> ProgressBar {
    let progress_bar = ProgressBar::new(len as u64).with_style(
        ProgressStyle::with_template("{elapsed_precise} {wide_bar} {pos}/{len} ({eta})").unwrap(),
    );
    progress_bar.enable_steady_tick(Duration::new(0, 100000000));
    progress_bar
}
