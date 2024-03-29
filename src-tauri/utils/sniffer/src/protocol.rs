use serde::{Deserialize, Serialize};

pub const PROTOCOL_DIR: &str = "dofus/protocol";

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
