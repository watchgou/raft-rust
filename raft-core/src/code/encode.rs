use protobuf::Message;
use tokio_util::codec::Encoder;

use super::{VoteEncode, MAX_SIZE};

impl<T: Message> Encoder<T> for VoteEncode<T> {
    type Error = std::io::Error;

    fn encode(&mut self, vote: T, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        match vote.write_to_bytes() {
            Ok(item) => {
                if item.len() > MAX_SIZE {
                    log::error!("Frame of length {} is too large.", item.len());

                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Frame of length {} is too large.", item.len()),
                    ));
                }
                let len_slice = u32::to_le_bytes(item.len() as u32);

                dst.reserve(4 + item.len());

                dst.extend_from_slice(&len_slice);
                dst.extend_from_slice(&item);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
