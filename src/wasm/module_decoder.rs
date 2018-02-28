use wasm::wasm_module::WasmModule;
use wasm::wasm_section::SectionType;
use std::mem;

static SECTION_CODE_TO_TYPE: [SectionType; 12] = [
    SectionType::Custom,
    SectionType::Type,
    SectionType::Import,
    SectionType::Function,
    SectionType::Table,
    SectionType::Memory,
    SectionType::Global,
    SectionType::Export,
    SectionType::Start,
    SectionType::Element,
    SectionType::Code,
    SectionType::Data,
];

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
        module.magic_number = read_byte_sequence_u32(self.bytes, &mut self.pos);
        module.version = read_byte_sequence_u32(self.bytes, &mut self.pos);
    }

    pub fn decode_section(&mut self) {
        let section_code = self.decode_section_code();
        self.decode_section_payload_len();

        match section_code {
            Some(&SectionType::Custom) => println!("found custom section"),
            Some(&SectionType::Type) => println!("found type section"),
            None => panic!("Invalid section code {:?}", section_code),
            _ => println!("found other section"),
        };
    }

    fn decode_section_code(&mut self) -> Option<&SectionType> {
        let id: usize = decode_leb128(self.bytes, &mut self.pos, false) as usize;
        SECTION_CODE_TO_TYPE.get(id)
    }

    fn decode_section_payload_len(&mut self) -> u32{
        let payload_len: u32 = decode_leb128(self.bytes, &mut self.pos, false) as u32;
        println!("section payload_len {:?}", payload_len);
        payload_len
    }

    fn decode_section_name_len(&mut self) -> u32 {
        let name_len: u32 = decode_leb128(self.bytes, &mut self.pos, false) as u32;
        name_len
    }
}

pub fn decode_leb128<'a>(bytes: &'a Vec<u8>, pos: &mut usize, signed: bool) -> u64 {
    let mut result: u64 = 0;
    let mut shift: u8 = 0;
    loop {
        let b: u8 = bytes[*pos];
        result |= ((b & 0x7f) << shift) as u64;
        shift += 7;
        *pos += 1;
        if b & 0x80 == 0 {
            break;
        }
    }
    if signed && shift < 64 {
        result |= !(0 as u64) << shift;
    }
    result
}

fn read_byte_sequence_u32<'a>(bytes: &'a Vec<u8>, pos: &mut usize) -> u32 {
    let a = [bytes[*pos], bytes[*pos + 1], bytes[*pos + 2], bytes[*pos + 3]];
    *pos += 4;
    unsafe {
        mem::transmute::<[u8; 4], u32>(a)
    }
}

// will be removed
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
