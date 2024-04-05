use bytes::BytesMut;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio_util::codec::Encoder;

use crate::{
    code::{VoteEncode, MAX_SIZE},
    protos::election::RequestVote,
};

pub async fn vote_client(leader_host: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream: TcpStream = TcpStream::connect(leader_host).await?;
    let mut ref_vote = RequestVote::default();
    ref_vote.candidateId = "".to_string();
    ref_vote.lastLogIndex = 1;
    ref_vote.lastLogTerm = 1;
    ref_vote.term = 1;
    let mut encode = VoteEncode::<RequestVote>::default();
    let mut buf = BytesMut::with_capacity(MAX_SIZE);
    let _ = encode.encode(ref_vote, &mut buf);
    let _ = stream.write_all(&buf);
    Ok(())
}
