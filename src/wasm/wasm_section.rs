use wasm::wasm_types;
use wasm::section_decoder;

type SectionDecoderType = fn(&Vec<u8>, usize) -> SectionPayload;

const SECTION_DECODERS: [SectionDecoderType; 12] = [
    section_decoder::decode_custom,
    section_decoder::decode_type,
    section_decoder::decode_import,
    section_decoder::decode_function,
    section_decoder::decode_table,
    section_decoder::decode_memory,
    section_decoder::decode_global,
    section_decoder::decode_export,
    section_decoder::decode_start,
    section_decoder::decode_element,
    section_decoder::decode_code,
    section_decoder::decode_data,
];

fn function generate_decoder(code) -> Box<Fn(&Vec<u8>, usize) -> SectionPayload> {
    match code {
        0 => 
    }
}

#[derive(Debug)]
pub struct SectionEntries<T> {
    entries: Vec<T>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum SectionType {
    Unknow(u8),
    Custom(u8),
    Type(u8),
    Import(u8),
    Function(u8),
    Table(u8),
    Memory(u8),
    Global(u8),
    Export(u8),
    Start(u8),
    Element(u8),
    Code(u8),
    Data(u8),
}

pub struct Section {
    pub section_code: u8,
    decoder:  Box<Fn(&Vec<u8>, usize) -> SectionPayload>,
    name: String,
    offset: usize,
    payload_len: usize,
    payload: SectionPayload,
}

impl Section {
    pub fn new(section_code: u8, name: String, offset: usize, payload_len: usize) -> Self {
        let decoder = &SECTION_DECODERS[section_code as usize];

        Section {
            section_code,
            name,
            offset,
            payload_len,
            payload: SectionPayload::Unknow,
            decoder: Box::new(move |bytes: &Vec<u8>, start: usize| -> SectionPayload {
                (decoder)(bytes, start)
            }),
        }
    }

    pub fn decode(&self, bytes: &Vec<u8>) -> SectionPayload {
        (self.decoder)(bytes, self.offset)
    }

    pub fn set_payload(&mut self, payload: SectionPayload) {
        self.payload = payload;
    }

    pub fn position(&self) -> (usize, usize) {
        let start = self.offset;
        let end = self.offset + self.payload_len;
        (start, end)
    }

    pub fn code_to_type (i: u8) -> SectionType {
        match i {
            0 => SectionType::Custom(0),
            1 => SectionType::Type(1),
            2 => SectionType::Import(2),
            3 => SectionType::Function(3),
            4 => SectionType::Table(4),
            5 => SectionType::Memory(5),
            6 => SectionType::Global(6),
            7 => SectionType::Export(7),
            8 => SectionType::Start(8),
            9 => SectionType::Element(9),
            10 => SectionType::Code(10),
            11 => SectionType::Data(11),
            _ => SectionType::Unknow(0xff),
        }
    }
}

#[derive(Debug)]
pub enum SectionPayload {
    Unknow,
    Custom,
    Type(Vec<wasm_types::FuncType>),
    Import(Vec<ImportEntry>),
    Function(Vec<u32>),
    Table(Vec<wasm_types::TableType>),
    Memory(Vec<wasm_types::MemoryType>),
    Global(Vec<GlobalEntry>),
    Export(Vec<ExportEntry>),
    Start(u32),
    Element(Vec<ElemSegment>),
    Code(Vec<FunctionBody>),
    Data(Vec<DataSegment>),
}

#[derive(Debug)]
pub struct ImportEntry {
    pub module_str: String,
    pub field_str: String,
    pub kind: wasm_types::ExternalKind,
}

#[derive(Debug)]
pub struct FunctionSection {
    pub types: Vec<u32>,
}

#[derive(Debug)]
pub struct GlobalEntry {
    pub variable_type: wasm_types::GlobalType,
    pub init: wasm_types::InitExpr,
}

#[derive(Debug)]
pub struct ExportEntry {
    pub field: String,
    pub kind: wasm_types::ExternalKind,
    pub index: u32,
}

#[derive(Debug)]
pub struct StartSection {
    pub index: u32,
}

#[derive(Debug)]
pub struct ElemSegment {
    pub index: u32,
    pub offset: wasm_types::InitExpr,
    pub elems: Vec<u32>,
}

#[derive(Debug)]
pub struct LocalEntry {
    pub count: u32,
    pub variable_type: wasm_types::ValueType,
}

#[derive(Debug)]
pub struct FunctionBody {
    pub locals: Vec<LocalEntry>,
    pub code: Vec<u8>,
    pub end: u8,
}

#[derive(Debug)]
pub struct DataSegment {
    pub index: u32,
    pub offset: wasm_types::InitExpr,
    pub data: Vec<u8>,
}
