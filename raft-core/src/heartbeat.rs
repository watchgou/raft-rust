use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_util::codec::{Decoder, Encoder};

use crate::{
    code::{VoteDecode, VoteEncode, MAX_SIZE},
    protos::election::{RequestAppendEntries, ResponseAppendEntries},
};

pub(crate) async fn run(
    cluster: &mut Option<Vec<String>>,
    heart: RequestAppendEntries,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(vc) = cluster {
        for host in vc.iter() {
            let stream = TcpStream::connect(host).await;
            match stream {
                Ok(mut stream) => {
                    write(&mut stream, heart.clone()).await?;
                    read(&mut stream).await?;
                }
                Err(e) => {
                    log::error!("{} => connection failed : {:#?}", host, e);
                }
            }
        }
    }
    Ok(())
}

#[inline]
async fn read(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut read_buf = BytesMut::with_capacity(MAX_SIZE);
    stream.read_buf(&mut read_buf).await?;
    let mut decode = VoteDecode::<ResponseAppendEntries>::default();
    let _ = decode.decode(&mut read_buf.clone())?;
    Ok(())
}

#[inline]
async fn write(
    stream: &mut TcpStream,
    heart: RequestAppendEntries,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut encode = VoteEncode::<RequestAppendEntries>::default();
    let mut write_buf = BytesMut::with_capacity(MAX_SIZE);
    encode.encode(heart, &mut write_buf)?;
    stream.write_all(&write_buf).await?;
    Ok(())
}
