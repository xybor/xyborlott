use chrono::Utc;

use crate::commitment::{Commitment, ToBytes};

#[derive(Clone)]
pub struct DrawPeriod {
    pub id: u64,
    pub start_time: chrono::DateTime<Utc>,
    pub prize: u64,
    pub prev_commitment: Commitment,
}

impl DrawPeriod {
    pub fn new(id: u64, prize: u64, prev: Commitment) -> Self {
        Self {
            id,
            start_time: Utc::now(),
            prize,
            prev_commitment: prev,
        }
    }
}

impl ToString for DrawPeriod {
    fn to_string(&self) -> String {
        format!("{} ({}$)", self.start_time, self.prize)
    }
}

impl ToBytes for DrawPeriod {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.id.to_be_bytes());
        bytes.extend(self.start_time.to_bytes());
        bytes.extend(self.prize.to_be_bytes());
        bytes.extend(self.prev_commitment);

        bytes
    }
}
