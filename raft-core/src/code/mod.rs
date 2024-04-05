pub mod decode;
pub mod encode;

pub static MAX_SIZE: usize = 8 * 1024 * 1024;

#[derive(Default)]
pub struct VoteDecode<T>(T);

#[derive(Default)]
pub struct VoteEncode<T>(T);
