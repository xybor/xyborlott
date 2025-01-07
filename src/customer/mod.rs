use crate::commitment::{Commitment, ToBytes};

pub mod phone;

pub trait Customer: ToString {
    fn commit(&self) -> Commitment;
    fn verify(&self, id: &dyn ToBytes) -> bool;
}
