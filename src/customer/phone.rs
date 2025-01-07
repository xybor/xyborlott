use chrono::Utc;

use crate::commitment::{commit, Commitment, ToBytes};

use super::Customer;

pub struct PhoneCustomer {
    time: chrono::DateTime<Utc>,
    commitment: Commitment,
}

impl PhoneCustomer {
    pub fn from(time: chrono::DateTime<Utc>, commitment: Commitment) -> Self {
        Self { time, commitment }
    }

    pub fn from_phone(id: String) -> Self {
        let time = Utc::now();

        let mut bytes = time.to_bytes();
        bytes.extend(id.to_bytes());

        Self {
            time,
            commitment: commit(&bytes.as_slice()),
        }
    }
}

impl ToString for PhoneCustomer {
    fn to_string(&self) -> String {
        format!(
            "{} (hidden={})",
            self.commitment
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>(),
            format!("{:02x}", self.time.timestamp_nanos_opt().unwrap()),
        )
    }
}

impl Customer for PhoneCustomer {
    fn commit(&self) -> Commitment {
        self.commitment
    }

    fn verify(&self, id: &dyn ToBytes) -> bool {
        let mut bytes = self.time.to_bytes();
        bytes.extend(id.to_bytes());

        self.commitment == commit(&bytes)
    }
}
