#[derive(Clone, Debug)]
pub struct Ed25519Signature(pub [u8; 64]);

#[derive(Clone, Debug)]
pub struct HsmSignature(pub Vec<u8>);
