use sha2::{Digest, Sha256};

use chrono::{DateTime, TimeZone};

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl<T: ToBytes + ?Sized> ToBytes for Box<T> {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_ref().to_bytes()
    }
}

impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ToBytes for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![self.clone()]
    }
}

impl ToBytes for Vec<u8> {
    fn to_bytes(&self) -> Vec<u8> {
        self.clone()
    }
}

impl ToBytes for [u8] {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl ToBytes for &[u8] {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl<Tz: TimeZone> ToBytes for DateTime<Tz> {
    fn to_bytes(&self) -> Vec<u8> {
        self.timestamp_nanos_opt().unwrap().to_be_bytes().to_vec()
    }
}

pub type Commitment = [u8; 32];

pub fn default() -> Commitment {
    let p = [0; 32];
    p
}

pub fn commit(obj: &dyn ToBytes) -> Commitment {
    let mut hasher = Sha256::new();
    hasher.update(obj.to_bytes());
    hasher.finalize().into()
}
