use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;
use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct BincodeCodec<T> {
    _item: PhantomData<T>,
}

impl<T> BincodeCodec<T> {
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

impl<T> Default for BincodeCodec<T> {
    fn default() -> Self {
        BincodeCodec { _item: PhantomData }
    }
}

impl<T> Encoder<&T> for BincodeCodec<T>
where
    T: Serialize,
{
    type Error = BincodeCodecError;

    fn encode(&mut self, item: &T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        bincode::serialize_into(dst.writer(), item)?;

        Ok(())
    }
}

impl<T> Decoder for BincodeCodec<T>
where
    T: DeserializeOwned,
{
    type Item = T;
    type Error = BincodeCodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor = std::io::Cursor::new(&src);
        match bincode::deserialize_from(&mut cursor) {
            Ok(bundles) => {
                src.advance(cursor.position().try_into().unwrap());

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
    use proptest::{prop_assert_eq, proptest};
    use proptest_derive::Arbitrary;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Arbitrary)]
    struct TestMessage {
        a: u32,
        b: bool,
        c: u128,
        d: String,
        e: TestEnum,
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Arbitrary)]
    enum TestEnum {
        A,
        B,
        C,
    }

    #[test]
    fn roundtrip_ok_some() {
        let mut codec = BincodeCodec::<TestMessage>::default();
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
        let mut codec = BincodeCodec::<TestMessage>::default();
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
        let mut codec = BincodeCodec::<TestMessage>::default();
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

    #[test]
    fn roundtrip_fuzz() {
        proptest!(|(msg: TestMessage)| {
            let mut codec = BincodeCodec::<TestMessage>::default();

            // Encode the message.
            let mut buffer = BytesMut::default();
            codec.encode(&msg, &mut buffer).unwrap();

            // Decode the message.
            let recovered = codec.decode(&mut buffer).unwrap().unwrap();
            prop_assert_eq!(recovered, msg);
        });
    }
}
