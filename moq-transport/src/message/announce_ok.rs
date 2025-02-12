use crate::coding::{Decode, DecodeError, Encode, EncodeError, Tuple};

/// Sent by the subscriber to accept an Announce.
#[derive(Clone, Debug)]
pub struct AnnounceOk {
    // Echo back the namespace that was announced.
    // TODO Propose using an ID to save bytes.
    pub namespace: Tuple,
}

impl Decode for AnnounceOk {
    fn decode<R: bytes::Buf>(r: &mut R) -> Result<Self, DecodeError> {
        let namespace = Tuple::decode(r)?;
        Ok(Self { namespace })
    }
}

impl Encode for AnnounceOk {
    fn encode<W: bytes::BufMut>(&self, w: &mut W) -> Result<(), EncodeError> {
        self.namespace.encode(w)
    }
}
