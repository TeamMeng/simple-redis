use super::{extract_fixed_data, RespDecode, RespEncode, RespError};
use bytes::BytesMut;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct RespNull;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct RespNullArray;

impl RespDecode for RespNullArray {
    const PREFIX: &'static str = "*";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "*-1\r\n", "NullArray")?;
        Ok(RespNullArray)
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(4)
    }
}

impl RespDecode for RespNull {
    const PREFIX: &'static str = "_";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "_\r\n", "Null")?;
        Ok(RespNull)
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(3)
    }
}

// - null array: "*-1\r\n"
impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

// - null: "_\r\n"
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;
    use anyhow::Result;

    #[test]
    fn test_null_array_encode() {
        let frame: RespFrame = RespNullArray.into();
        assert_eq!(frame.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null_encode() {
        let frame: RespFrame = RespNull.into();
        assert_eq!(frame.encode(), b"_\r\n");
    }

    #[test]
    fn test_null_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*-1\r\n");

        let frame = RespNullArray::decode(&mut buf)?;
        assert_eq!(frame, RespNullArray);

        Ok(())
    }

    #[test]
    fn test_null_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"_\r\n");

        let frame = RespNull::decode(&mut buf)?;
        assert_eq!(frame, RespNull);

        Ok(())
    }
}
