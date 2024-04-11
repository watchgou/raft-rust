pub mod decode;
pub mod encode;

pub const MAX_SIZE: usize = 0x800000;

#[derive(Default)]
pub struct VoteDecode<T>(T);

#[derive(Default)]
pub struct VoteEncode<T>(T);
