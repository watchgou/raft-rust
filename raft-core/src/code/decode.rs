use super::VoteDecode;
use super::MAX_SIZE;
use bytes::Buf;
use protobuf::Message;
use std::io::Error;
use tokio_util::codec::Decoder;

impl<T: Message> Decoder for VoteDecode<T> {
    type Item = T;
    type Error = Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }
        let mut length_bytes = [0u8; 4];

        length_bytes.copy_from_slice(&src[..4]);
        let length = u32::from_le_bytes(length_bytes) as usize;
        if length > MAX_SIZE {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Frame of length {} is too large.", length),
            ));
        }

        if src.len() < 4 + length {
            src.reserve(4 + length - src.len());
            return Ok(None);
        }
        let result = src[4..4 + length].to_vec();
        src.advance(4 + length);

        match T::parse_from_bytes(&result) {
            Ok(vote) => Ok::<std::option::Option<T>, std::io::Error>(Some(vote)),
            Err(e) => Err(e.into()),
        }
    }
}
