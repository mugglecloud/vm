const WASM_MAGIC_NUMBER: u32 = 0x6d736100;
const WASM_VERSION: u32 = 0x01;

use wasm::wasm_section::Section;

#[derive(Debug)]
pub struct WasmModule {
    pub magic_number: u32,
    pub version: u32,
    pub sections: Vec<Section>,
}

impl WasmModule {
    pub fn new() -> WasmModule {
        WasmModule { 
            magic_number: WASM_MAGIC_NUMBER,
            version: WASM_VERSION,
            sections: Vec::new()
        }
    }
}