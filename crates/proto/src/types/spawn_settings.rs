use bedrockrs_proto_macros::ProtoCodec;

use bedrockrs_shared::world::dimension::Dimension;

use crate::types::spawn_biome_type::SpawnBiomeType;

#[derive(ProtoCodec, Debug, Clone)]
pub struct SpawnSettings {
    pub biome_type: SpawnBiomeType,
    pub user_defined_biome_name: String,
    pub dimension: Dimension,
}
