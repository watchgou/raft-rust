use bytes::BytesMut;
use tokio::net::TcpStream;

use tokio::{io::AsyncReadExt, net::TcpListener};
use tokio_util::codec::Decoder;

use crate::code::VoteDecode;
use crate::code::MAX_SIZE;
use crate::protos::election::RequestVote;

pub async fn vote_server(host_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(host_name).await?;
    loop {
        if let Ok((mut stream, address)) = listener.accept().await {
            tokio::spawn(async move {
                log::info!("addrees: {}", address);
                let mut buf = bytes::BytesMut::with_capacity(MAX_SIZE);
                let _ = stream.read_buf(&mut buf);
                vote_handle(&mut buf, &stream).await;
            });
        } else {
            continue;
        }
    }
}

//#[inline]
async fn vote_handle(bytes: &mut BytesMut, stream: &TcpStream) {
    let mut decode = VoteDecode::<RequestVote>::default();
    match decode.decode(bytes) {
        Ok(result) => {
            if let Some(value) = result {
                // 处理投票

                value.lastLogIndex;
            }
        }
        Err(e) => {
            log::error!("{}", e);
        }
    }
}
