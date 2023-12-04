

#[derive(PartialEq, Debug, clap::ValueEnum, Clone)]
pub enum Endianness {
    Little,
    Big
}