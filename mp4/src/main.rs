use std::{fs, io::{Write, Cursor}, ops::Deref};

use crate::merge::join_buffers;

mod desc_reader;
mod writer;
mod merge;

fn main() {
    // mp4_merge::join_files(&[
    //     "condor-1.mp4",
    //     "condor-2.mp4",
    //     "condor-3.mp4",
    // ], "condor-merged.mp4", |p| println!("progress: {p}"))
    //     .unwrap();

    let files = [
        "flying-dog-main-test-1.mp4",
        "flying-dog-main-test-2.mp4",
    ]
    .map(|path| fs::read(path).unwrap())
    .to_vec();
    
    let merged = join_buffers(files).unwrap();
    println!("result: {}", merged.len());

    let mut f_out = fs::File::create("merged.mp4").unwrap();
    f_out.write_all(merged.deref()).unwrap();
}
