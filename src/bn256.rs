use crate::fq::*;
use crate::fr::*;
pub use crate::curve::*;

pub type Base = Fr;
pub type Scalar = Fq;
pub type Point = Bn256G1;
pub type Affine = Bn256G1Affine;
pub type Compressed = Bn256G1Compressed;

/*
pub use halo2curves::bn256;
pub type Base = bn256::Fq;
pub type Scalar = bn256::Fr;
pub type Point = bn256::G1;
pub type Affine = bn256::G1Affine;
pub type Compressed = bn256::G1Compressed;
*/
