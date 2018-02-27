#[derive(Debug)]
pub enum TypeConstructorScheme {
    I32,
    I64,
    F32,
    F64,
    AnyFunc,
    Func,
    EmptyType,
}

#[derive(Debug)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug)]
pub enum ElemType {
    AnyFunc,
}

#[derive(Debug)]
pub enum BlockType {
    ValueType,
    EmptyType,
}

#[derive(Debug)]
pub struct FuncType {
    form: i8,
    param: Vec<ValueType>,
    ret: Vec<ValueType>,
}

#[derive(Debug)]
pub struct GlobalType {
    content_type: ValueType,
    mutability: u8,
}

#[derive(Debug)]
pub struct TableType {
    element_type: ElemType,
    limits: ResizableLimits,
}

#[derive(Debug)]
pub struct MemoryType {
    limits: ResizableLimits,
}

#[derive(Debug)]
pub enum ExternalKind {
    Function,
    Table,
    Memory,
    Global,
}

#[derive(Debug)]
pub struct ResizableLimits {
    flags: u8,
    initial: u32,
    maximun: u32
}

#[derive(Debug)]
pub struct InitExpr {
    
}
