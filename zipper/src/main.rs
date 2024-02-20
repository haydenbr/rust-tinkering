use std::{io::{Cursor, Write}, fs};
use zip::{ZipWriter, write::FileOptions};

fn main() {
    let mut data = Vec::new();

    write_zip_files(&mut data);

    fs::write("zipper.zip", data).unwrap();
}

fn write_zip_files(data: &mut Vec<u8>) {
    let buff = std::io::Cursor::new(data);

    let mut zw = zip::ZipWriter::new(buff);

    for i in 0..10 {
        zw.start_file(format!("{}.txt", i), FileOptions::default()).unwrap();
        zw.write_all(format!("Hello from file {}!", i).as_bytes()).unwrap();
    }
    
    zw.finish().unwrap();
}
