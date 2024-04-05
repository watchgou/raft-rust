use std::time::Duration;

use bytes::BytesMut;
use tokio::net::TcpStream;

use tokio::time::timeout;
use tokio::{io::AsyncReadExt, net::TcpListener};
use tokio_util::codec::Decoder;

use crate::code::VoteDecode;
use crate::code::MAX_SIZE;
use crate::protos::election::RequestVote;
use crate::RaftConfig;

pub async fn vote_server(conf: &RaftConfig) {
    if let Some(host_name) = &conf.host_names {
        let listener = TcpListener::bind(host_name).await.unwrap();
        loop {
            let accept_timeout = timeout(
                Duration::from_millis(conf.out_time.unwrap()),
                listener.accept(),
            );
            match accept_timeout.await {
                Ok(result) => match result {
                    Ok((mut stream, address)) => {
                        tokio::spawn(async move {
                            log::info!("address: {}", address);
                            let mut buf = bytes::BytesMut::with_capacity(MAX_SIZE);
                            let _ = stream.read_buf(&mut buf);
                            vote_handle(&mut buf, &stream).await;
                        });
                    }
                    Err(e) => {
                        log::error!(" {}", e);
                    }
                },
                Err(e) => {
                    log::info!("modify statue :{}", e);
                    break;
                }
            }
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
