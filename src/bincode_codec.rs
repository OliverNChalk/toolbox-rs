use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;
use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct BincodeCodec<const BUFFER: usize, T> {
    buffer: [u8; BUFFER],
    _item: PhantomData<T>,
}

impl<const BUFFER: usize, T> BincodeCodec<BUFFER, T> {
    fn check_bincode_error(err: Box<bincode::ErrorKind>) -> Result<(), BincodeCodecError> {
        match err.as_ref() {
            bincode::ErrorKind::Io(io_err) => match io_err.kind() {
                std::io::ErrorKind::UnexpectedEof => Ok(()),
                _ => Err(err),
            },
            _ => Err(err),
        }
        .map_err(Into::into)
    }
}

impl<const BUFFER: usize, T> Default for BincodeCodec<BUFFER, T> {
    fn default() -> Self {
        BincodeCodec { buffer: [0; BUFFER], _item: PhantomData }
    }
}

impl<const BUFFER: usize, T> Encoder<T> for BincodeCodec<BUFFER, T>
where
    T: Serialize,
{
    type Error = BincodeCodecError;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // Serialize into intermediate buffer.
        let mut cursor = std::io::Cursor::new(&mut self.buffer[..]);
        bincode::serialize_into(&mut cursor, &item)?;

        // Flush to dst.
        let end = cursor.position() as usize;
        dst.put(&self.buffer[0..end]);

        Ok(())
    }
}

impl<const BUFFER: usize, T> Encoder<&T> for BincodeCodec<BUFFER, T>
where
    T: Serialize,
{
    type Error = BincodeCodecError;

    fn encode(&mut self, item: &T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // Serialize into intermediate buffer.
        let mut cursor = std::io::Cursor::new(&mut self.buffer[..]);
        bincode::serialize_into(&mut cursor, item)?;

        // Flush to dst.
        let end = cursor.position() as usize;
        dst.put(&self.buffer[0..end]);

        Ok(())
    }
}

impl<const BUFFER: usize, T> Decoder for BincodeCodec<BUFFER, T>
where
    T: DeserializeOwned,
{
    type Item = T;
    type Error = BincodeCodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor = std::io::Cursor::new(&src);
        match bincode::deserialize_from(&mut cursor) {
            Ok(bundles) => {
                src.advance(cursor.position() as usize);

                Ok(Some(bundles))
            }
            Err(err) => Self::check_bincode_error(err).map(|_| None),
        }
    }
}

#[derive(Debug, Error)]
pub enum BincodeCodecError {
    #[error("Io error; err={0}")]
    Io(#[from] std::io::Error),
    #[error("Deserialization error; err={0}")]
    Deserialization(#[from] Box<bincode::ErrorKind>),
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct TestMessage {
        a: u32,
        b: bool,
        c: u128,
        d: String,
        e: TestEnum,
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    enum TestEnum {
        A,
        B,
        C,
    }

    #[test]
    fn roundtrip_ok_some() {
        let mut codec = BincodeCodec::<4096, TestMessage>::default();
        let original =
            TestMessage { a: 0, b: true, c: 2, d: "hello world".to_string(), e: TestEnum::A };

        // Encode the message.
        let mut buffer = BytesMut::default();
        codec.encode(&original, &mut buffer).unwrap();

        // Decode the message.
        let recovered = codec.decode(&mut buffer).unwrap().unwrap();
        assert_eq!(recovered, original);
    }

    #[test]
    fn roundtrip_ok_none() {
        let mut codec = BincodeCodec::<4096, TestMessage>::default();
        let original =
            TestMessage { a: 0, b: true, c: 2, d: "hello world".to_string(), e: TestEnum::A };

        // Encode the message.
        let mut buffer = BytesMut::default();
        codec.encode(&original, &mut buffer).unwrap();

        // Drop the last byte of the message.
        buffer.truncate(buffer.len() - 1);

        // Decode the message.
        let recovered = codec.decode(&mut buffer).unwrap();
        assert_eq!(recovered, None);
    }

    #[test]
    fn roundtrip_err() {
        let mut codec = BincodeCodec::<4096, TestMessage>::default();
        let original =
            TestMessage { a: 0, b: true, c: 2, d: "hello world".to_string(), e: TestEnum::A };

        // Encode the message.
        let mut buffer = BytesMut::default();
        codec.encode(&original, &mut buffer).unwrap();

        // Make the enum an invalid variant (LE encoded so any non-zero would
        // technically do the trick).
        let len = buffer.len();
        buffer[len - 1] = u8::MAX;

        // Decode the message.
        assert!(codec.decode(&mut buffer).is_err());
    }
}
