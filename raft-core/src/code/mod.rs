pub mod decode;
pub mod encode;

pub static MAX_SIZE: usize = 8 * 1024 * 1024;

#[derive(Default)]
pub struct RequestVoteDcode;

#[derive(Default)]
pub struct ResponseVoteDecode;

#[derive(Default)]
pub struct RequestVoteEncode;

#[derive(Default)]
pub struct ResponseVoteEncode;
