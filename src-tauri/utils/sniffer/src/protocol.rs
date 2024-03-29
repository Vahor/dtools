use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref PROTOCOL_FILE: String = format!("{}/protocol.json", PROTOCOL_DIR);
    pub static ref TRANSLATIONS_DIR: String = format!("{}/translations.json", PROTOCOL_DIR);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProtocolVarType {
    UTF,
    VarUhShort,
    VarShort,
    Short,
    Float,
    VarUhLong,
    VarLong,
    Byte,
    VarUhInt,
    Int,
    Double,
    Boolean,
    UnsignedInt,
    UnsignedShort,
    VarInt,
    UnsignedByte,
    ByteArray,
    False,
}
