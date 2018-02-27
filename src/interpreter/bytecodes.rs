// #[derive(Debug)]
// enum AccumulatorUse {
//     Read,
//     Write,
//     None
// }

// #[derive(Debug)]
// struct Bytecode {
//     value: u8,
//     name: String,
//     operands: Vec<u8>,
//     accumulator_use: AccumulatorUse,
// }

#[derive(Debug)]
pub enum V8BytecodeList<T> {
    // header start: 00
    Header(T),

    // base address 
    Base(T),

    Wide(T),
    ExtraWide(T),

    // loading the accumulator
    LdaZero(T),
    LdaSmi(T),
    LdaUndefined(T),
    LdaNull(T),
    LdaTheHole(T),
    LdaTrue(T),
    LdaFalse(T),
    LdaConstant(T),

    // Globals
    LdaGlobal(T),
    LdaGlobalInsideTypeof(T),
    StaGlobalSloppy(T),
    StaGlobalStrict(T),

    // Context operations
    PushContext(T),
    PopContext(T),
    LdaContextSlot(T),
    LdaImmutableContextSlot(T),
    LdaCurrentContextSlot(T),
    LdaImmutableCurrentContextSlot(T),
    StaContextSlot(T),
    StaCurrentContextSlot(T),

    // Load-Store lookup slots
    LdaLookupSlot(T),
    LdaLookupContextSlot(T),
    LdaLookupGlobalSlot(T),
    LdaLookupSlotInsideTypeof(T),
    LdaLookupContextSlotInsideTypeof(T),
    LdaLookupGlobalSlotInsideTypeof(T),
    StaLookupSlot(T),

    // Register-accumulator transfers
    Ldar(T),
    Star(T),

    // Register-register transfers
    Mov(T),

    // Property loads (LoadIC) operations
    LdaNamedProperty(T),
    LdaKeyedProperty(T),

    // Operations on module variables
    LdaModuleVariable(T),
    StaModuleVariable(T),

    // Propery stores (StoreIC) operations
    StaNamedProperty(T),
    StaNamedOwnProperty(T),
    StaKeyedProperty(T),
    StaDataPropertyInLiteral(T),
    CollectTypeProfile(T),

    // Binary Operators
    Add(T),
    Sub(T),
    Mul(T),
    Div(T),
    Mod(T),
    BitwiseOr(T),
    BitwiseXor(T),
    BitwiseAnd(T),
    ShiftLeft(T),
    ShiftRight(T),
    ShiftRightLogical(T),

    // Binary operators with immediate operands
    AddSmi(T),
    SubSmi(T),
    MulSmi(T),
    DivSmi(T),
    ModSmi(T),
    BitwiseOrSmi(T),
    BitwiseXorSmi(T),
    BitwiseAndSmi(T),
    ShiftLeftSmi(T),
    ShiftRightSmi(T),
    ShiftRightLogicalSmi(T),

    // Unary Operators
    Inc(T),
    Dec(T),
    Negate(T),
    BitwiseNot(T),
    ToBooleanLogicalNot(T),
    LogicalNot(T),
    TypeOf(T),
    DeletePropertyStrict(T),
    DeletePropertySloppy(T),

    // GetSuperConstructor operator
    GetSuperConstructor(T),

    // Call operations
    CallAnyReceiver(T),
    CallProperty(T),
    CallProperty0(T),
    CallProperty1(T),
    CallProperty2(T),
    CallUndefinedReceiver(T),
    CallUndefinedReceiver0(T),
    CallUndefinedReceiver1(T),
    CallUndefinedReceiver2(T),
    CallWithSpread(T),
    CallRuntime(T),
    CallRuntimeForPair(T),
    CallJSRuntime(T),

    // Intrinsics
    InvokeIntrinsic(T),

    // Construct operators
    Construct(T),
    ConstructWithSpread(T),

    // Test Operators
    TestEqual(T),
    TestEqualStrict(T),
    TestLessThan(T),
    TestGreaterThan(T),
    TestLessThanOrEqual(T),
    TestGreaterThanOrEqual(T),
    TestEqualStrictNoFeedback(T),
    TestInstanceOf(T),
    TestIn(T),
    TestUndetectable(T),
    TestNull(T),
    TestUndefined(T),
    TestTypeOf(T),

    // Cast operators
    ToName(T),
    ToNumber(T),
    ToNumeric(T),
    ToObject(T),

    // Literals
    CreateRegExpLiteral(T),
    CreateArrayLiteral(T),
    CreateEmptyArrayLiteral(T),
    CreateObjectLiteral(T),
    CreateEmptyObjectLiteral(T),

    // Tagged templates
    GetTemplateObject(T),

    // Closure allocation
    CreateClosure(T),

    // Context allocation
    CreateBlockContext(T),
    CreateCatchContext(T),
    CreateFunctionContext(T),
    CreateEvalContext(T),
    CreateWithContext(T),

    // Arguments allocation
    CreateMappedArguments(T),
    CreateUnmappedArguments(T),
    CreateRestParameter(T),

    // Control Flow -- carefully ordered for efficient checks
    // - [Unconditional jumps]
    JumpLoop(T),
    // - [Forward jumps]
    Jump(T),
    // - [Start constant jumps]
    JumpConstant(T),
    // - [Conditional jumps]
    // - [Conditional constant jumps]
    JumpIfNullConstant(T),
    JumpIfNotNullConstant(T),
    JumpIfUndefinedConstant(T),
    JumpIfNotUndefinedConstant(T),
    JumpIfTrueConstant(T),
    JumpIfFalseConstant(T),
    JumpIfJSReceiverConstant(T),
    // - [Start ToBoolean jumps]
    JumpIfToBooleanTrueConstant(T),
    JumpIfToBooleanFalseConstant(T),
    // - [End constant jumps]
    // - [Conditional immediate jumps]
    JumpIfToBooleanTrue(T),
    JumpIfToBooleanFalse(T),
    // - [End ToBoolean jumps]
    JumpIfTrue(T),
    JumpIfFalse(T),
    JumpIfNull(T),
    JumpIfNotNull(T),
    JumpIfUndefined(T),
    JumpIfNotUndefined(T),
    JumpIfJSReceiver(T),

    // Smi-table lookup for switch statements
    SwitchOnSmiNoFeedback(T),

    // Complex flow control For..in
    ForInEnumerate(T),
    ForInPrepare(T),
    ForInContinue(T),
    ForInNext(T),
    ForInStep(T),

    // Perform a stack guard check
    StackCheck(T),

    // Update the pending message
    SetPendingMessage(T),

    // Non-local flow control
    Throw(T),
    ReThrow(T),
    Return(T),
    ThrowReferenceErrorIfHole(T),
    ThrowSuperNotCalledIfHole(T),
    ThrowSuperAlreadyCalledIfNotHole(T),

    // Generators
    RestoreGeneratorState(T),
    SuspendGenerator(T),
    RestoreGeneratorRegisters(T),

    // Debugger
    Debugger(T),

    // Debug Breakpoints - one for each possible size of unscaled bytecodes
    // and one for each operand widening prefix bytecode
    DebugBreak0(T),
    DebugBreak1(T),
    DebugBreak2(T),
    DebugBreak3(T),
    DebugBreak4(T),
    DebugBreak5(T),
    DebugBreak6(T),
    DebugBreakWide(T),
    DebugBreakExtraWide(T),

    // Block Coverage
    IncBlockCounter(T),

    // Execution Abort (internal error)
    Abort(T),

    // Illegal bytecode
    Illegal(T)
}

// pub fn to_bytecode(arg: u8) -> BytecodeList<u8> {
//     unimplemented!();
// }
