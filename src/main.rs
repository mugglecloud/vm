extern crate turbofan;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use turbofan::wasm::module_decoder::WasmModuleDecoder;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents: Vec<u8> = Vec::new();
    let size = f.read_to_end(&mut contents)
        .expect("something went wrong reading the file");

    println!("read {} bytes", size);

    let mut decoder = WasmModuleDecoder::new(&contents);
    decoder.decode_module_header();
    
    decoder.decode_section();

    println!("{:?}", decoder.module());
}