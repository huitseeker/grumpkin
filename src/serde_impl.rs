use ff::PrimeField;
use group::GroupEncoding;
use ff;
use serde::{
    de::Error as DeserializeError, Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{Fq, Fr, G1, G1Affine, G1Compressed, Bn256G1, Bn256G1Affine, Bn256G1Compressed};
use group::Curve;

/// Serializes bytes to human readable or compact representation.
///
/// Depending on whether the serializer is a human readable one or not, the bytes are either
/// encoded as a hex string or a list of bytes.
fn serialize_bytes<S: Serializer>(bytes: [u8; 32], s: S) -> Result<S::Ok, S::Error> {
    if s.is_human_readable() {
        hex::serde::serialize(bytes, s)
    } else {
        bytes.serialize(s)
    }
}

/// Deserialize bytes from human readable or compact representation.
///
/// Depending on whether the deserializer is a human readable one or not, the bytes are either
/// decoded from a hex string or a list of bytes.
fn deserialize_bytes<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 32], D::Error> {
    if d.is_human_readable() {
        hex::serde::deserialize(d)
    } else {
        <[u8; 32]>::deserialize(d)
    }
}

impl Serialize for Fq {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        serialize_bytes(self.to_repr(), s)
    }
}

impl<'de> Deserialize<'de> for Fq {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = deserialize_bytes(d)?;
        match Fq::from_repr(bytes).into() {
            Some(fq) => Ok(fq),
            None => Err(D::Error::custom(
                "deserialized bytes don't encode a Pallas field element",
            )),
        }
    }
}

impl Serialize for Fr {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        serialize_bytes(self.to_repr(), s)
    }
}

impl<'de> Deserialize<'de> for Fr {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = deserialize_bytes(d)?;
        match Fr::from_repr(bytes).into() {
            Some(fr) => Ok(fr),
            None => Err(D::Error::custom(
                "deserialized bytes don't encode a Bn254 field element",
            )),
        }
    }
}

impl Serialize for G1Affine {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        serialize_bytes(self.to_bytes().0, s)
    }
}

impl<'de> Deserialize<'de> for G1Affine {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = deserialize_bytes(d)?;
        let com = G1Compressed(bytes);
        match G1Affine::from_bytes(&com).into() {
            Some(g1_affine) => Ok(g1_affine),
            None => Err(D::Error::custom(
                "deserialized bytes don't encode a Grumpkin curve point",
            )),
        }
    }
}

impl Serialize for Bn256G1Affine {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        serialize_bytes(self.to_bytes().0, s)
    }
}

impl<'de> Deserialize<'de> for Bn256G1Affine {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = deserialize_bytes(d)?;
        let com = Bn256G1Compressed(bytes);
        match Bn256G1Affine::from_bytes(&com).into() {
            Some(g1_affine) => Ok(g1_affine),
            None => Err(D::Error::custom(
                "deserialized bytes don't encode a Grumpkin curve point",
            )),
        }
    }
}

impl Serialize for G1 {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        G1Affine::serialize(&self.to_affine(), s)
    }
}

impl<'de> Deserialize<'de> for G1 {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(G1Affine::deserialize(d)?))
    }
}

impl Serialize for Bn256G1 {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        Bn256G1Affine::serialize(&self.to_affine(), s)
    }
}

impl<'de> Deserialize<'de> for Bn256G1 {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(Bn256G1Affine::deserialize(d)?))
    }
}

impl Serialize for G1Compressed {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for G1Compressed {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self(<[u8; 32]>::deserialize(d)?))
    }
}

impl Serialize for Bn256G1Compressed {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Bn256G1Compressed {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self(<[u8; 32]>::deserialize(d)?))
    }
}
