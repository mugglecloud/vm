// use std::thread;
use std::mem;
// use std::sync::RwLock;
use std::sync::Arc;
use std::time::{ Instant };

use wasm::wasm_module::WasmModule;
// use wasm::wasm_section::SectionType;
use wasm::wasm_section::Section;
// use wasm::section_decoder;
// use wasm::wasm_section::SectionPayload;

use share::leb128::decode_leb128;
// use share::thread_pool::ThreadPool;

pub struct WasmModuleDecoder {
    module: WasmModule,
    pos: usize,
    bytes: Arc<Vec<u8>>,
    // thread_pool: ThreadPool,
}

impl WasmModuleDecoder {
    pub fn new(bytes: Vec<u8>) -> WasmModuleDecoder {
        WasmModuleDecoder { 
            module: WasmModule::new(),
            pos: 0,
            bytes: Arc::new(bytes)
        }
    }

    pub fn module(self) -> WasmModule {
        self.module
    }
    
    pub fn offset(&self) -> usize {
        self.pos
    }

    pub fn decode_module_header(&mut self) {
        let module: &mut WasmModule = &mut self.module;
        let magic_number = read_byte_sequence_u32(&self.bytes, &mut self.pos);
        if module.magic_number != magic_number {
            panic!("invalid magic number {:?}", magic_number);
        }
        module.version = read_byte_sequence_u32(&self.bytes, &mut self.pos);
    }

    pub fn decode_section(&mut self) {
        let size = self.bytes.len();

        let now = Instant::now();
        loop {
            if self.pos == size {
                break;
            }
            let section = self.get_section();

            self.module.sections.insert(section.section_code, section);
        }
        println!("decode section meta: time elaspsed {:?}μs", now.elapsed().subsec_nanos() / 1000);

        let now = Instant::now();
        self.decode_payload();
        println!("decode section: time elaspsed {:?}μs", now.elapsed().subsec_nanos() / 1000);
    }

    pub fn decode_payload (&mut self) {
        // let (tx, rx) = mpsc::channel();
        let sections = &mut self.module.sections;
        for (_, s) in sections {
            // let b = Arc::clone(&self.bytes);
            // let b = &self.bytes;
            s.decode(&self.bytes);
            
            // println!("section {:?}", s.section_code);
        }
    }

    fn get_section(&mut self) -> Section {
        let section_code = self.decode_section_code();
        let payload_len = self.decode_section_payload_len() as usize;
        let start = self.pos;

        let mut section_name: String = String::new();

        if section_code == 0 {
            // custom section
            section_name = self.decode_custom();
        }
        let offset = self.pos;
        self.pos = start + payload_len;


        Section::new(section_code, section_name, offset, payload_len)
    }

    fn decode_custom(&mut self) -> String {
        let (_, name) = self.decode_section_name();
        name
    }

    fn decode_section_code(&mut self) -> u8 {
        decode_leb128(&self.bytes, &mut self.pos, false) as u8
    }

    fn decode_section_payload_len(&mut self) -> u32 {
        let payload_len = decode_leb128(&self.bytes, &mut self.pos, false);
        payload_len as u32
    }

    fn decode_section_name(&mut self) -> (usize, String) {
        let start = self.pos;
        let bytes = &self.bytes;
        let name_len = decode_leb128(bytes, &mut self.pos, false) as usize;
        let mut name = String::with_capacity(name_len);
        for i in 0..name_len {
            name.push(bytes[start + i] as char);
        }
        self.pos += name_len;
        (name_len, name)
    }
}

fn read_byte_sequence_u32<'a>(bytes: &'a Vec<u8>, pos: &mut usize) -> u32 {
    let a = [bytes[*pos], bytes[*pos + 1], bytes[*pos + 2], bytes[*pos + 3]];
    *pos += 4;
    unsafe {
        mem::transmute::<[u8; 4], u32>(a)
    }
}

