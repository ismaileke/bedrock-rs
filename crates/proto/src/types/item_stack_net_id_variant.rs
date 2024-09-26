use bedrockrs_core::int::VAR;
use bedrockrs_macros::ProtoCodec;
#[derive(ProtoCodec, Debug, Clone)]
pub struct ItemStackNetIdVariant {
    pub raw_id: VAR<i32>,
}
