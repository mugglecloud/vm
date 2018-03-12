extern crate turbofan;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use turbofan::wasm::module_decoder::WasmModuleDecoder;

fn main() {
    let total_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let now = Instant::now();
    let mut f = File::open(filename).expect("file not found");
    println!("open file: time elaspsed {:?}μs", now.elapsed().subsec_nanos() / 1000);
    let mut bytes: Vec<u8> = Vec::new();

    let now = Instant::now();
    let size = f.read_to_end(&mut bytes)
        .expect("something went wrong reading the file");
    println!("read {} bytes", size);
    println!("read wasm file: time elaspsed {:?}μs", now.elapsed().subsec_nanos() / 1000);

    let mut decoder = WasmModuleDecoder::new(bytes);

    decoder.decode_module_header();
    decoder.decode_section();

    // println!("{:?}", decoder.module());
    println!("total execute time: time elaspsed {:?}μs", total_time.elapsed().subsec_nanos() / 1000);
}