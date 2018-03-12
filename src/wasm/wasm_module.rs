use std::collections::HashMap;

const WASM_MAGIC_NUMBER: u32 = 0x6d736100;
const WASM_VERSION: u32 = 0x01;

use wasm::wasm_section;

pub struct WasmModule {
    pub magic_number: u32,
    pub version: u32,
    pub sections: HashMap<u8, wasm_section::Section>,
}

impl WasmModule {
    pub fn new() -> WasmModule {
        WasmModule { 
            magic_number: WASM_MAGIC_NUMBER,
            version: WASM_VERSION,
            sections: HashMap::new()
        }
    }
}