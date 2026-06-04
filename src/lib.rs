pub mod tools;
pub mod v1;
pub mod v2;
pub mod v3;
pub mod v4;
pub mod v5;
pub mod v6;

use type_equalities::{IsEqual, trivial_eq};

use crate::tools::{hash_vec, read::*, write::*};

pub trait Track
where
    // It would be more correct to enforce equality of the `Track` instead of the `Track::Metadata`.
    // But to avoid a one-time-use `TypeFunction`, it is done this way
    Self::Metadata: IsEqual<<<<Self::Part as Part>::Block as Block>::Track as Track>::Metadata>,
{
    type Part: Part;

    type Metadata: Copy;
    fn meta(&self) -> Self::Metadata;
    fn parts(&self) -> Vec<Self::Part>;

    fn decode_meta(data: &[u8], offset: &mut usize) -> Option<Self::Metadata>;
    fn encode_meta(&self, data: &mut Vec<u8>);
    fn from_data(metadata: Self::Metadata, parts: Vec<Self::Part>) -> Self;

    type TrackInfo;
    fn decode_track_code(
        track_code: &str,
    ) -> Option<(
        /* name */ String,
        Self::TrackInfo,
        /* track_data */ Vec<u8>,
    )>;
    fn encode_track_code(name: String, info: Self::TrackInfo, track_data: &[u8]) -> Option<String>;
}
pub trait Part: Sized {
    type Block: Block;

    // ASSUMPTION: All PolyTrack version's `Part` consist of only these three data (holds true atleast upto 0.6.2)
    // If assumption is broken, this trait will need a bit of a rewrite
    fn id(&self) -> u8;
    fn amount(&self) -> u32;
    fn blocks(&self) -> Vec<Self::Block>;
    fn from_data(id: u8, amount: u32, blocks: Vec<Self::Block>) -> Self;

    fn decode_prelude(data: &[u8], offset: &mut usize) -> Option<(u8, u32)>;
    fn encode_prelude(&self, data: &mut Vec<u8>);
    fn decode(
        data: &[u8],
        offset: &mut usize,
        meta: <<Self::Block as Block>::Track as Track>::Metadata,
    ) -> Option<Self> {
        let (id, amount) = Self::decode_prelude(data, offset)?;
        let mut blocks = Vec::new();
        for _ in 0..amount {
            blocks.push(Self::Block::decode(data, offset, id, meta)?);
        }
        Some(Self::from_data(id, amount, blocks))
    }
    fn encode(&self, data: &mut Vec<u8>, meta: <<Self::Block as Block>::Track as Track>::Metadata) {
        self.encode_prelude(data);
        for block in &self.blocks() {
            block.encode(data, meta);
        }
    }
}
pub trait Block: Sized {
    type Track: Track;

    type Extra;
    fn extra_data(&self) -> Self::Extra;

    // because it changes i32 -> u32 in v4 -> v5
    type Coord;
    fn pos(&self) -> (Self::Coord, Self::Coord, Self::Coord);
    fn rot(&self) -> u8;

    fn decode(
        data: &[u8],
        offset: &mut usize,
        id: u8,
        meta: <Self::Track as Track>::Metadata,
    ) -> Option<Self>;
    fn encode(&self, data: &mut Vec<u8>, meta: <Self::Track as Track>::Metadata);
}

pub fn decode_track_data<T: Track>(data: &[u8]) -> Option<T> {
    let mut offset = 0;
    // ASSUMPTION: All metadata lives before the main data (holds true for v5 and v6, which are the only ones with nontrivial metadata)
    let meta = T::decode_meta(data, &mut offset)?;
    let mut parts = Vec::new();
    while offset < data.len() {
        parts.push(T::Part::decode(
            data,
            &mut offset,
            trivial_eq::<
                // Rust should be able to figure this out itself smh.
                _,
                <<<T::Part as Part>::Block as Block>::Track as Track>::Metadata,
            >()
            .coerce(meta),
        )?);
    }

    Some(T::from_data(meta, parts))
}
pub fn encode_track_data<T: Track>(track: &T) -> Vec<u8> {
    let mut data = Vec::new();
    let meta = track.meta();

    // ASSUMPTION: All metadata lives before the main data (holds true for v5 and v6, which are the only ones with nontrivial metadata)
    track.encode_meta(&mut data);
    for part in track.parts() {
        part.encode(
            &mut data,
            trivial_eq::<
                // Rust should be able to figure this out itself smh.
                _,
                <<<T::Part as Part>::Block as Block>::Track as Track>::Metadata,
            >()
            .coerce(meta),
        );
    }

    data
}
pub fn export_to_id<T: Track>(track_code: &str) -> Option<String> {
    let (_, _, track_data) = T::decode_track_code(track_code)?;
    let id = hash_vec(track_data);
    Some(id)
}
