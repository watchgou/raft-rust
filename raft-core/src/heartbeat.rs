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
        let mut write_buf = BytesMut::with_capacity(MAX_SIZE);
        let mut read_buf = BytesMut::with_capacity(MAX_SIZE);
        let mut encode = VoteEncode::<RequestAppendEntries>::default();
        let mut decode = VoteDecode::<ResponseAppendEntries>::default();
        for host in vc.iter() {
            let stream = TcpStream::connect(host).await;
            match stream {
                Ok(mut stream) => {
                    encode.encode(heart.clone(), &mut write_buf)?;
                     stream.write_all(&write_buf).await?;
                    stream.read_buf(&mut read_buf).await?;
                    heartbeat_result_handle(&mut read_buf, &mut decode).await?;
                    read_buf.clear();
                    write_buf.clear();
                }
                Err(e) => {
                    log::error!("{} => connection failed : {:#?}", host, e);
                }
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
async fn heartbeat_result_handle(
    read_buf: &mut BytesMut,
    decode: &mut VoteDecode<ResponseAppendEntries>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = decode.decode(&mut read_buf.clone())?;
    Ok(())
}
