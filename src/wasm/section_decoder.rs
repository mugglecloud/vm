use std::time::Instant;

// use share::log;

use wasm::wasm_section;
// use wasm::wasm_section::SectionType;
use wasm::wasm_section::SectionPayload;
use wasm::wasm_section::ImportEntry;

use wasm::wasm_types;
use wasm::wasm_types::FuncType;
use wasm::wasm_types::ValueType;
use wasm::wasm_types::ExternalKind;
use wasm::wasm_types::to_value_type;

// use wasm::operators;

use share::leb128::decode_leb128;

fn decode_section_entries<'a, T, F>(payload: &'a Vec<u8>, start: usize, decode: F) -> Vec<T> 
    where F: Fn(&Vec<u8>, &mut usize) -> T {
    let mut pos = start;
    let mut count: u32 = decode_leb128(payload, &mut pos, false) as u32;
    let mut entries: Vec<T> = Vec::with_capacity(count as usize);

    // println!("number of entries {:?}", count);

    loop {
        if count == 0 {
            break;
        }
        entries.push(decode(payload, &mut pos));
        count -= 1;
    }

    entries
}

pub fn decode_custom(payload: &Vec<u8>, start: usize) -> SectionPayload {
    SectionPayload::Custom
}

pub fn decode_type(payload: &Vec<u8>, start: usize) -> SectionPayload {
    let decode = |payload: &Vec<u8>, pos: &mut usize| {
        decode_func_type(payload, pos)
    };
    let entries: Vec<FuncType> = decode_section_entries(payload, start, decode);
    SectionPayload::Type(entries)
}

pub fn decode_import(payload: &Vec<u8>, start: usize) -> SectionPayload {
    let decode = |payload: &Vec<u8>, pos: &mut usize| {
        decode_import_entry(payload, pos)
    };
    let entries: Vec<ImportEntry> = decode_section_entries(payload, start, decode);
    SectionPayload::Import(entries)
}

pub fn decode_function(payload: &Vec<u8>, start: usize) -> SectionPayload {
    let decode = |payload: &Vec<u8>, pos: &mut usize| {
        decode_leb128(payload, pos, false) as u32
    };
    let entries: Vec<u32> = decode_section_entries(payload, start, decode);
    SectionPayload::Function(entries)
}

pub fn decode_table(payload: &Vec<u8>, start: usize) -> SectionPayload {
    let decode = |payload: &Vec<u8>, pos: &mut usize| {
        deocde_table_type(payload, pos)
    };
    let entries: Vec<wasm_types::TableType> = decode_section_entries(payload, start, decode);
    SectionPayload::Table(entries)
}

pub fn decode_memory(payload: &Vec<u8>, start: usize) -> SectionPayload {
    let decode = |payload: &Vec<u8>, pos: &mut usize| {
        decode_memory_type(payload, pos)
    };
    let entries: Vec<wasm_types::MemoryType> = decode_section_entries(payload, start, decode);
    SectionPayload::Memory(entries)
}

pub fn decode_global(payload: &Vec<u8>, start: usize) -> SectionPayload {
    let decode = |payload: &Vec<u8>, pos: &mut usize| {
        decode_global_entry(payload, pos)
    };
    let entries: Vec<wasm_section::GlobalEntry> = decode_section_entries(payload, start, decode);
    SectionPayload::Global(entries)
}

pub fn decode_export(payload: &Vec<u8>, start: usize) -> SectionPayload {
    let decode = |payload: &Vec<u8>, pos: &mut usize| {
        decode_export_entry(payload, pos)
    };
    let entries: Vec<wasm_section::ExportEntry> = decode_section_entries(payload, start, decode);
    SectionPayload::Export(entries)
}

pub fn decode_start(payload: &Vec<u8>, start: usize) -> SectionPayload {
    let mut pos = start;
    let index = decode_leb128(payload, &mut pos, false) as u32;
    SectionPayload::Start(index)
}

pub fn decode_element(payload: &Vec<u8>, start: usize) -> SectionPayload {
    let decode = |payload: &Vec<u8>, pos: &mut usize| {
        decode_element_entry(payload, pos)
    };
    let entries: Vec<wasm_section::ElemSegment> = decode_section_entries(payload, start, decode);
    SectionPayload::Element(entries)
}

pub fn decode_code(payload: &Vec<u8>, start: usize) -> SectionPayload {
    let now = Instant::now();
    let decode = |payload: &Vec<u8>, pos: &mut usize| {
        decode_function_body(payload, pos)
    };
    let entries: Vec<wasm_section::FunctionBody> = decode_section_entries(payload, start, decode);
    println!("decode code: time elaspsed {:?}μs", now.elapsed().subsec_nanos() / 1000);
    SectionPayload::Code(entries)
}

pub fn decode_data(payload: &Vec<u8>, start: usize) -> SectionPayload {
    let decode = |payload: &Vec<u8>, pos: &mut usize| {
        decode_data_segment(payload, pos)
    };
    let entries: Vec<wasm_section::DataSegment> = decode_section_entries(payload, start, decode);
    SectionPayload::Data(entries)
}

fn decode_data_segment<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_section::DataSegment {
    let index = decode_leb128(payload, pos, false) as u32;
    let offset = decode_init_expr(payload, pos);
    let size = decode_leb128(payload, pos, false) as usize;
    let mut data: Vec<u8> = vec![];
    let start: usize = *pos;

    *pos += size;

    // println!("data segment size {:?}, @ {:?}", size, start);

    data.extend_from_slice(&payload[start..*pos]);

    wasm_section::DataSegment {
        index,
        offset,
        data,
    }
}

fn decode_function_body<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_section::FunctionBody {
    let body_size = decode_leb128(payload, pos, false) as usize;
    let start: usize = *pos;
    let end: u8 = 0x0b;
    let locals = decode_locals(payload, pos);

    // println!("functon body size {:?}, header size {:?}", body_size, *pos - start);

    let end_pos = start + body_size;
    let mut code: Vec<u8> = Vec::with_capacity(body_size - (*pos - start));
    code.extend_from_slice(&payload[*pos..end_pos]);

    // println!("code size {:?}, last byte {:?}", end_pos - *pos, payload[end_pos - 1]);

    *pos = end_pos;

    wasm_section::FunctionBody {
        locals,
        code,
        end,
    }
}

fn decode_locals<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> Vec<wasm_section::LocalEntry> {
    let local_count = decode_leb128(payload, pos, false) as usize;
    // println!("local entry count {:?}", local_count);
    let mut locals: Vec<wasm_section::LocalEntry> = Vec::with_capacity(local_count);
    for _i in 0..local_count {
        locals.push(decode_local_entry(payload, pos));
    }

    locals
}

fn decode_local_entry<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_section::LocalEntry {
    let count = decode_leb128(payload, pos, false) as u32;
    let variable_type = decode_value_type(payload, pos);

    wasm_section::LocalEntry {
        count,
        variable_type,
    }
}

fn decode_element_entry<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_section::ElemSegment {
    let index = decode_leb128(payload, pos, false) as u32;
    let offset = decode_init_expr(payload, pos);
    let elems: Vec<u32> = decode_vec_u32(payload, pos);

    wasm_section::ElemSegment {
        index,
        offset,
        elems,
    }
}

fn decode_export_entry<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_section::ExportEntry {
    let field = decode_string(payload, pos);
    let kind = decode_external_kind(payload, pos);
    let index = decode_leb128(payload, pos, false) as u32;

    wasm_section::ExportEntry {
        field,
        kind,
        index,
    }
}

fn decode_init_expr<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_types::InitExpr {
    let expression = decode_bytecode(payload, pos, 0x0b);

    println!("init expr {:?}", expression);

    wasm_types::InitExpr {
        expression,
    }
}

fn decode_global_entry<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_section::GlobalEntry {
    let variable_type = decode_global_type(payload, pos);
    let init = decode_init_expr(payload, pos);

    wasm_section::GlobalEntry {
        variable_type,
        init,
    }
}

fn decode_func_type<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> FuncType {
    let form = decode_leb128(payload, pos, true) as i8;
    let param_count = decode_leb128(payload, pos, false) as u32;
    let mut param_types: Vec<ValueType> = Vec::with_capacity(param_count as usize);
    
    for _i in 0..param_count {
        let t = decode_leb128(payload, pos, true) as i8;
        param_types.push(to_value_type(t));
    }

    let return_count = decode_leb128(payload, pos, false) as u8;
    let mut return_types: Vec<ValueType> = Vec::with_capacity(return_count as usize);

    for _i in 0..return_count {
        let r = decode_leb128(payload, pos, true) as i8;
        return_types.push(to_value_type(r));
    }

    FuncType {
        form,
        param_types,
        return_types,
    }
}

fn decode_import_entry<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> ImportEntry {
    let module_str = decode_string(payload, pos);
    let field_str = decode_string(payload, pos);
    let kind = decode_external_kind(payload, pos);

    ImportEntry {
        module_str,
        field_str,
        kind,
    }
}

fn decode_external_kind<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> ExternalKind {
    let kind = decode_leb128(payload, pos, false) as u8;
    match kind {
        0 => ExternalKind::Function(decode_leb128(payload, pos, false) as u32),
        1 => ExternalKind::Table(deocde_table_type(payload, pos)),
        2 => ExternalKind::Memory(decode_memory_type(payload, pos)),
        3 => ExternalKind::Global(decode_global_type(payload, pos)),
        _ => ExternalKind::Unknow,
    }
}

fn decode_value_type<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_types::ValueType {
    let v = decode_leb128(payload, pos, true) as i8;
    to_value_type(v)
}

fn decode_global_type<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_types::GlobalType {
    let content_type = decode_value_type(payload, pos);
    let mutability = decode_leb128(payload, pos, false) as u8;

    wasm_types::GlobalType {
        content_type,
        mutability,
    }
}

fn decode_memory_type<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_types::MemoryType {
    let limits = decode_resizable_limits(payload, pos);

    wasm_types::MemoryType {
        limits,
    }
}

fn deocde_table_type<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_types::TableType {
    let element_type = decode_leb128(payload, pos, false) as i8;
    let limits = decode_resizable_limits(payload, pos);

    wasm_types::TableType {
        element_type,
        limits,
    }
}

fn decode_resizable_limits<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> wasm_types::ResizableLimits {
    let flags = decode_leb128(payload, pos, false) as u8;
    let initial = decode_leb128(payload, pos, false) as u32;
    let maximun = if let 1 = flags {
        Some(decode_leb128(payload, pos, false) as u32)
    } else {
        None
    };

    wasm_types::ResizableLimits {
        flags,
        initial,
        maximun,
    }
}

fn decode_string<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> String {
    let start = *pos;
    let module_len = decode_leb128(payload, pos, false) as usize;
    let mut module_str: String = String::with_capacity(module_len);

    *pos += module_len;

    for i in start..*pos {
        module_str.push(payload[i] as char);
    }
    module_str
}

fn decode_vec_u32<'a>(payload: &'a Vec<u8>, pos: &mut usize) -> Vec<u32> {
    let num_len = decode_leb128(payload, pos, false) as usize;
    let mut elems: Vec<u32> = Vec::with_capacity(num_len);

    for _i in 0..num_len {
        elems.push(decode_leb128(payload, pos, false) as u32);
    }
    elems
}

fn decode_bytecode<'a>(payload: &'a Vec<u8>, pos: &mut usize, end: u8) -> Vec<u8> {
    let mut start = *pos;
    let mut code: u8 = payload[start];
    let mut expression: Vec<u8> = vec![];

    loop {
        expression.push(code);
        start += 1;
        // stop and wasm operator <end: 0x0b>
        if code == end {
            break;
        }
        code = payload[start];
    }
    
    *pos = start;

    expression
}
