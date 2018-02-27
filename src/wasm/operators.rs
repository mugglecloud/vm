#[derive(Debug)]
struct BytecodeEntry<I>() {
    name: String,
    opcode: u8,
    immediates: Option<I>,
    description: String,
}

#[derive(Debug)]
pub enum BytecodeList<T> {
    // control flow operators
    Unreachable(T),
    Nop(T),
    Block(T),
    Loop(T),
    If(T),
    Else(T),
    End(T),
    Br(T),
    BrIf(T),
    BrTable(T),
    Return(T),

    // call operators
    Call(T),
    CallIndirect(T),

    // parametric operators
    Drop(T),
    Select(T),

    // variable access
    GetLocal(T),
    SetLocal(T),
    TeeLocal(T),
    GetGlobal(T),
    SetGlobal(T),

    // memory-related operators
    I32Load(T),
    I64Load(T),
    F32Load(T),
    F64Load(T),
    I32Load8s(T),
    I32Load8u(T),
    I32Load16s(T),
    I32Load16u(T),
    I64Load8s(T),
    I64Load8u(T),
    I64Load16s(T),
    I64Load16u(T),
    I64Load32s(T),
    I64Load32u(T),
    I32Store(T),
    I64Store(T),
    F32Store(T),
    F64Store(T),
    I32Store8(T),
    I32Store16(T),
    I64Store8(T),
    I64Store16(T),
    I64Store32(T),
    CurrentMemory(T),
    GrowMemory(T),

    // constants
    I32Const(T),
    I64Const(T),
    F32Const(T),
    F64Const(T),

    // comparison operators

    // numeric operators

    // conversions

    // reinterpretations
}