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

    let mut decoder = WasmModuleDecoder::new();
    let bytes: Box<Vec<u8>> = Box::new(contents);
    decoder.decode_module_header(bytes);
    let module = decoder.module();

    println!("{:?}", module);

    println!("{:?} {}", 0x7f & 0x82, 0x01 << 8);
}