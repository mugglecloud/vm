#[derive(Debug)]
pub enum ValueType {
    Unknow,
    I32(i8),
    I64(i8),
    F32(i8),
    F64(i8),
    AnyFunc(i8),
    Func(i8),
    BlockType(i8),
}

#[derive(Debug)]
pub enum ElemType {
    AnyFunc(i8),
}

#[derive(Debug)]
pub enum BlockType {
    ValueType(i8),
    EmptyType,
}

#[derive(Debug)]
pub struct FuncType {
    pub form: i8,
    pub param_types: Vec<ValueType>,
    pub return_types: Vec<ValueType>,
}

#[derive(Debug)]
pub struct GlobalType {
    pub content_type: ValueType,
    pub mutability: u8,
}

#[derive(Debug)]
pub struct TableType {
    pub element_type: i8,
    pub limits: ResizableLimits,
}

#[derive(Debug)]
pub struct MemoryType {
    pub limits: ResizableLimits,
}

#[derive(Debug)]
pub enum ExternalKind {
    Unknow,
    Function(u32),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}

#[derive(Debug)]
pub struct ResizableLimits {
    pub flags: u8,
    pub initial: u32,
    pub maximun: Option<u32>
}

#[derive(Debug)]
pub struct InitExpr {
    pub expression: Vec<u8>,
}

pub fn to_value_type(t: i8) -> ValueType {
    match t {
        -0x01 => ValueType::I32(t),
        -0x02 => ValueType::I64(t),
        -0x03 => ValueType::F32(t),
        -0x04 => ValueType::F64(t),
        -0x10 => ValueType::AnyFunc(t),
        -0x20 => ValueType::Func(t),
        -0x40 => ValueType::BlockType(t),
        _ => ValueType::Unknow,
    }
}

