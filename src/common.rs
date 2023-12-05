#[derive(PartialEq, Debug, Clone, clap::ValueEnum)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(PartialEq, Debug, Clone, clap::ValueEnum)]
pub enum DataType {
    I8,
    I16,
    I32,
    I64,
    I128,

    U8,
    U16,
    U32,
    U64,
    U128,

    F32,
    F64,
}
