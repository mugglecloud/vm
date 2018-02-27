use wasm::wasm_module::WasmModule;
use std::mem;

#[derive(Debug)]
pub struct WasmModuleDecoder {
    module: WasmModule,
    pos: usize,
    size: usize,
}

impl WasmModuleDecoder {
    pub fn new() -> WasmModuleDecoder {
        WasmModuleDecoder { 
            module: WasmModule::new(),
            pos: 0,
            size: 0,
        }
    }

    pub fn module(self) -> WasmModule {
        self.module
    }

    pub fn decode_module_header(&mut self, bytes: Box<Vec<u8>>) {
        self.size = bytes.len();
        let module: &mut WasmModule = &mut self.module;
        module.magic_number = read_byte_sequence_u32(&bytes, self.pos);
        self.pos += 4;
        module.version = read_byte_sequence_u32(&bytes, self.pos);
        self.pos += 4;
    }

    pub fn decode_section() {

    }
}

#[derive(Debug)]
enum _ByteSquence {
    U32,
    U64,
    Str,
}

fn leb128_remove_sign_extend(b: u8) -> u8 {
    let mut shift = 2;
    let sign = b & 0x40;
    while (b << shift) & 0x80 == sign {
        shift += 1;
    }
}

pub fn decode_leb128(bytes: &Box<Vec<u8>>, pos: usize, signed: bool) -> u64 {
    let mut index = pos;
    let mut result: u64 = 0;
    let mut shift: u8 = 0;
    while true {
        let mut b: u8 = bytes[index];
        if b & 0x80 == 0 {
            if signed {
                b = leb128_remove_sign_extend(b);
            }
            result |= (b & 0x7f) << shift;
            break;
        }
        result |= (b & 0x7f) << shift;
        index += 1;
        shift += 7;
    }
    result
}

fn _read_little_endian(bytes: &Box<Vec<u8>>, pos: usize, num: usize) -> Vec<u8> {
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

fn read_byte_sequence_u32(bytes: &Box<Vec<u8>>, pos: usize) -> u32 {
    let a = [bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]];
    unsafe {
        mem::transmute::<[u8; 4], u32>(a)
    }
}