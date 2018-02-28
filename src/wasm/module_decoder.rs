use wasm::wasm_module::WasmModule;
use std::mem;

#[derive(Debug)]
enum _DataTypes {
    Uint8,
    Uint16,
    Uint32,
    Varuint1,
    Varuint7,
    Varuint32,
    Varint7,
    Varint32,
    Varint64,
}

#[derive(Debug)]
pub struct WasmModuleDecoder<'a> {
    module: WasmModule,
    pos: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> WasmModuleDecoder<'a> {
    pub fn new(bytes: &Vec<u8>) -> WasmModuleDecoder {
        WasmModuleDecoder { 
            module: WasmModule::new(),
            pos: 0,
            bytes: bytes
        }
    }

    pub fn module(self) -> WasmModule {
        self.module
    }

    pub fn decode_module_header(&mut self) {
        let module: &mut WasmModule = &mut self.module;
        module.magic_number = read_byte_sequence_u32(self.bytes, self.pos);
        self.pos += 4;
        module.version = read_byte_sequence_u32(self.bytes, self.pos);
        self.pos += 4;
    }

    pub fn decode_section(&mut self) -> u8 {
        let id: u8 = decode_leb128(self.bytes, self.pos, false) as u8;
        self.pos += 1;
        id
    }
}

pub fn decode_leb128<'a>(bytes: &'a Vec<u8>, pos: usize, signed: bool) -> u64 {
    let mut index = pos;
    let mut result: u64 = 0;
    let mut shift: u8 = 0;
    loop {
        let b: u8 = bytes[index];
        result |= ((b & 0x7f) << shift) as u64;
        shift += 7;
        if b & 0x80 == 0 {
            break;
        }
        index += 1;
    }
    if signed && shift < 64 {
        result |= !(0 as u64) << shift;
    }
    result
}

fn _read_little_endian<'a>(bytes: &'a Vec<u8>, pos: usize, num: usize) -> Vec<u8> {
    let mut index = pos;
    let mut done = false;
    let mut a: Vec<u8> = Vec::new();
    let end = pos + num;
    while !done {
        let b = bytes[index];
        a.push(b);
        index += 1;
        if index == end {
            done = true;
        }
    }
    a
}

fn read_byte_sequence_u32<'a>(bytes: &'a Vec<u8>, pos: usize) -> u32 {
    let a = [bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]];
    unsafe {
        mem::transmute::<[u8; 4], u32>(a)
    }
}