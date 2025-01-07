use crate::commitment::{commit, ToBytes};
use bcrypt;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

#[derive(Debug, Default, Clone)]
pub struct TicketNumber6Of55 {
    p: [u8; 6],
}

impl TicketNumber6Of55 {
    pub fn from(p: [u8; 6]) -> Option<Self> {
        for pi in p {
            if pi == 0 || pi > 55 {
                return None;
            }
        }

        Some(Self { p })
    }

    pub fn random(seed: u64) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(seed);

        let mut number = TicketNumber6Of55::default();
        for i in 0..number.p.len() {
            number.p[i] = rng.gen_range(1..=55);
        }

        number
    }
}

impl PartialEq for TicketNumber6Of55 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..6 {
            if self.p[i] != other.p[i] {
                return false;
            }
        }

        true
    }
}

impl Eq for TicketNumber6Of55 {}

impl ToBytes for TicketNumber6Of55 {
    fn to_bytes(&self) -> Vec<u8> {
        self.p.to_vec()
    }
}

impl ToString for TicketNumber6Of55 {
    fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.p[0], self.p[1], self.p[2], self.p[3], self.p[4], self.p[5],
        )
    }
}

impl crate::ticket_number::TicketNumber for TicketNumber6Of55 {
    fn draw_from(_: &[Self], source: &[u8]) -> Vec<Self> {
        let hash_value = commit(&bcrypt::hash(source, 15).unwrap());
        let mut rng = ChaCha20Rng::from_seed(hash_value);

        let mut number = TicketNumber6Of55::default();
        for i in 0..number.p.len() {
            number.p[i] = rng.gen_range(1..=55);
        }

        vec![number]
    }
}
