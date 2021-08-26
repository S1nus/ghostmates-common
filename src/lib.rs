use async_std::{
    net::{TcpStream}
};

use futures_codec::{Bytes, BytesMut, LengthCodec, Framed, FramedRead, FramedWrite, Decoder, Encoder};
use std::io::{Error, ErrorKind};
use async_std::io::{BufWriter, BufReader};

pub struct StringCodec(pub LengthCodec);

pub fn new_codec_reader(stream: BufReader<&TcpStream>) -> FramedRead<BufReader<&TcpStream>, StringCodec> {
    FramedRead::new(stream, StringCodec(LengthCodec))
}

pub fn new_codec_writer(stream: BufWriter<&TcpStream>) -> FramedWrite<BufWriter<&TcpStream>, StringCodec> {
    FramedWrite::new(stream, StringCodec(LengthCodec))
}

impl Encoder for StringCodec {
    type Item = String;
    type Error = Error;

    fn encode(&mut self, src: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = Bytes::from(src);
        self.0.encode(bytes, dst)
    }
}

impl Decoder for StringCodec {
    type Item = String;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.0.decode(src)? {
            Some(bytes) => {
                match String::from_utf8(bytes.to_vec()) {
                    Ok(string) => Ok(Some(string)),
                    Err(e) => Err(Error::new(ErrorKind::InvalidData, e))
                }
            },
            None => Ok(None),
        }
    }
}
