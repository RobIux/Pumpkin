use generation::proto_chunk::ProtoChunk;
use pumpkin_util::math::vector2::Vector2;

pub mod biome;
pub mod block;
pub mod chunk;
pub mod coordinates;
pub mod cylindrical_chunk_iterator;
pub mod dimension;
mod generation;
pub mod item;
pub mod level;
mod lock;
pub mod loot;
mod noise_router;
pub mod world_info;
pub const WORLD_HEIGHT: usize = 384;
pub const WORLD_LOWEST_Y: i16 = -64;
pub const WORLD_MAX_Y: i16 = WORLD_HEIGHT as i16 - WORLD_LOWEST_Y.abs();
pub const DIRECT_PALETTE_BITS: u32 = 15;

#[macro_export]
macro_rules! read_data_from_file {
    ($path:expr) => {
        serde_json::from_str(
            &fs::read_to_string(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join(file!())
                    .parent()
                    .unwrap()
                    .join($path),
            )
            .expect("no data file"),
        )
        .expect("failed to decode data")
    };
}

// TODO: is there a way to do in-file benches?
pub use generation::{
    noise_router::proto_noise_router::GlobalProtoNoiseRouter, GlobalRandomConfig,
};
pub use noise_router::NOISE_ROUTER_ASTS;

pub fn bench_create_and_populate_noise(
    base_router: &GlobalProtoNoiseRouter,
    random_config: &GlobalRandomConfig,
) {
    let mut chunk = ProtoChunk::new(Vector2::new(0, 0), base_router, random_config);
    chunk.populate_noise();
}
