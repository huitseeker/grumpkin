use crate::fq::*;
use crate::fr::*;
pub use crate::curve::*;

pub type Base = Fq;
pub type Scalar = Fr;
pub type Point = G1;
pub type Affine = G1Affine;
pub type Compressed = G1Compressed;

/*
pub use halo2curves::bn256::{Fr, Fq};
pub type Base = Fr;
pub type Scalar = Fq;
pub type Point = G1;
pub type Affine = G1Affine;
*/
