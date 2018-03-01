use wasm::wasm_types;

#[derive(Debug)]
pub struct SectionEntries<T> {
    entries: Vec<T>,
}

#[derive(Debug)]
pub enum SectionType {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
}

#[derive(Debug)]
pub struct Section {
    id: u8,
    name: String,
    payload: Vec<u8>,
}

#[derive(Debug)]
pub struct ImportEntry {
    module_str: String,
    field_str: String,
    kind: wasm_types::ExternalKind,
}

#[derive(Debug)]
pub struct FunctionSection {
    types: Vec<u32>,
}

#[derive(Debug)]
pub struct GlobalVariable {
    variable_type: wasm_types::GlobalType,
    init: wasm_types::InitExpr,
}

#[derive(Debug)]
pub struct ExportEntry {
    field: String,
    kind: wasm_types::ExternalKind,
    index: u32,
}

#[derive(Debug)]
pub struct StartSection {
    index: u32,
}

#[derive(Debug)]
pub struct ElemSegment {
    index: u32,
    offset: wasm_types::InitExpr,
    elems: Vec<u32>,
}

#[derive(Debug)]
pub struct LocalEntry {
    local_variables: Vec<wasm_types::ValueType>
}

#[derive(Debug)]
pub struct FunctionBody {
    locals: Vec<LocalEntry>,
    code: Vec<u8>,
    end: u8,
}

#[derive(Debug)]
pub struct DataSegment {
    index: u32,
    offset: wasm_types::InitExpr,
    data: Vec<u8>,
}
