// #[derive(Debug)]
// struct BytecodeEntry<I>() {
//     name: String,
//     opcode: u8,
//     immediates: Option<I>,
//     description: String,
// }

#[derive(Debug)]
pub enum BytecodeList<T = u8> {
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
    OpDrop(T),
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

pub fn to_bytecode(code: u8) -> BytecodeList {
    match code {
        // constrol flow operators
        0x00 => BytecodeList::Unreachable(0x00),
        0x01 => BytecodeList::Nop(0x01),
        0x02 => BytecodeList::Block(0x02),
        0x03 => BytecodeList::Loop(0x03),
        0x04 => BytecodeList::If(0x04),
        0x05 => BytecodeList::Else(0x05),
        0x0b => BytecodeList::End(0x0b),
        0x0c => BytecodeList::Br(0x0c),
        0x0d => BytecodeList::BrIf(0x0d),
        0x0e => BytecodeList::BrTable(0x0e),
        0x0f => BytecodeList::Return(0x0f),
        // call operators
        0x10 => BytecodeList::Call(0x10),
        0x11 => BytecodeList::CallIndirect(0x11),
        // parametric operators
        0x1a => BytecodeList::OpDrop(0x1a),
        0x1b => BytecodeList::Select(0x1b),
        // variable access
        0x20 => BytecodeList::GetLocal(0x20),
        0x21 => BytecodeList::SetLocal(0x21),
        0x22 => BytecodeList::TeeLocal(0x22),
        0x23 => BytecodeList::GetGlobal(0x23),
        0x24 => BytecodeList::SetGlobal(0x24),
        // memory related operators
        0x28 => BytecodeList::I32Load(0x28),
        0x29 => BytecodeList::I64Load(0x29),
        0x2a => BytecodeList::F32Load(0x2a),
        0x2b => BytecodeList::F64Load(0x2b),
        0x2c => BytecodeList::I32Load8s(0x2c),
        0x2d => BytecodeList::I32Load8u(0x2d),
        0x2e => BytecodeList::I32Load16s(0x2e),
        0x2f => BytecodeList::I32Load16u(0x2f),
        0x30 => BytecodeList::I64Load8s(0x30),
        0x31 => BytecodeList::I64Load8u(0x31),
        0x32 => BytecodeList::I64Load16s(0x32),
        0x33 => BytecodeList::I64Load16u(0x33),
        0x34 => BytecodeList::I64Load32s(0x34),
        0x35 => BytecodeList::I64Load32u(0x35),
        0x36 => BytecodeList::I32Store(0x36),
        0x37 => BytecodeList::I64Store(0x37),
        0x38 => BytecodeList::F32Store(0x38),
        0x39 => BytecodeList::F64Store(0x39),
        0x3a => BytecodeList::I32Store8(0x3a),
        0x3b => BytecodeList::I32Store16(0x3b),
        0x3c => BytecodeList::I64Store8(0x3c),
        0x3d => BytecodeList::I64Store16(0x3d),
        0x3e => BytecodeList::I64Store32(0x3e),
        0x3f => BytecodeList::CurrentMemory(0x3f),
        0x40 => BytecodeList::GrowMemory(0x40),
        _ => {
            panic!("invalid bytecode {:?}", code)
        },
    }
}