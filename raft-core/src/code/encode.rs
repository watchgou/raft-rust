use super::RequestVoteEncode;
use super::ResponseVoteEncode;
use super::MAX_SIZE;
use crate::protos::election::RequestVote;
use crate::protos::election::ResponseVote;
use protobuf::Message;
use std::io::Error;
use tokio_util::codec::Encoder;

impl Encoder<RequestVote> for RequestVoteEncode {
    type Error = Error;

    fn encode(&mut self, vote: RequestVote, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
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

impl Encoder<ResponseVote> for ResponseVoteEncode {
    type Error = Error;

    fn encode(&mut self, vote: ResponseVote, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
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
