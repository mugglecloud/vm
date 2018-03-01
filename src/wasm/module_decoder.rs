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
        let section_code = decode_section_code(self.bytes, &mut self.pos);
        let payload_len = decode_section_payload_len(self.bytes, &mut self.pos) as usize;

        println!("section payload_len {:?}", payload_len);

        let payload = match section_code {
            Some(&SectionType::Custom) => decode_custom(self.bytes, &mut self.pos, payload_len),
            Some(&SectionType::Type) => decode_type(self.bytes, &mut self.pos, payload_len),
            _ => Vec::new(),
        };

        println!("payload {:?}", payload);
    }
}

fn decode_custom<'a>(bytes: &'a Vec<u8>, pos: &mut usize, payload_len: usize) -> Vec<u8> {
    let (name_size, name) = decode_section_name(bytes, pos);
    print!("section name {:?}, consume {:?}", name, name_size);
    decode_section_payload(bytes, pos, payload_len - name_size)
}

fn decode_type<'a>(bytes: &'a Vec<u8>, pos: &mut usize, payload_len: usize) -> Vec<u8> {
    decode_section_payload(bytes, pos, payload_len)
}

pub fn decode_section_code<'a>(bytes: &'a Vec<u8>, pos: &mut usize) -> Option<&'a SectionType> {
    let id = decode_leb128(bytes, pos, false) as usize;
    SECTION_CODE_TO_TYPE.get(id)
}

pub fn decode_section_payload_len<'a>(bytes: &'a Vec<u8>, pos: &mut usize) -> u32 {
    let payload_len = decode_leb128(bytes, pos, false);
    payload_len as u32
}

pub fn decode_section_name<'a>(bytes: &'a Vec<u8>, pos: &mut usize) -> (usize, String) {
    let start = *pos;
    let mut name_len = decode_leb128(bytes, pos, false);
    let mut name = String::new();
    while name_len > 0 {
        name.push(bytes[*pos] as char);
        *pos += 1;
        name_len -= 1;
    }
    (*pos - start, name)
}

pub fn decode_section_payload<'a>(bytes: &'a Vec<u8>, pos: &mut usize, size: usize) -> Vec<u8> {
    let mut len = size;
    let mut payload: Vec<u8> = Vec::new();
    while len > 0 {
        payload.push(bytes[*pos]);
        *pos += 1;
        len -= 1;
    }
    payload
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
