use crate::commitment::{commit, ToBytes};
use bcrypt;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

#[derive(Debug, Default, Clone)]
pub struct TicketNumberLoto {
    p: [u8; 6],
}

impl TicketNumberLoto {
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

        let mut number = TicketNumberLoto::default();
        for i in 0..number.p.len() {
            number.p[i] = rng.gen_range(1..=55);
        }

        number
    }

    fn find_parital_win(layer: &[TicketNumberLoto], v: u8) -> Vec<usize> {
        let mut partial_wins = vec![];
        for i in 0..layer.len() {
            if layer[i].p.contains(&v) {
                partial_wins.push(i)
            }
        }

        partial_wins
    }

    fn remove_winners(
        layer: &mut Vec<TicketNumberLoto>,
        partial_wins: Vec<usize>,
    ) -> Vec<TicketNumberLoto> {
        let mut winners = vec![];
        let removed = layer
            .iter()
            .enumerate()
            .filter(|(i, x)| {
                let contain = partial_wins.contains(i);
                if contain {
                    winners.push((*x).clone());
                }
                !contain
            })
            .map(|(_, x)| x.clone())
            .collect();

        (*layer) = removed;

        winners
    }
}

impl PartialEq for TicketNumberLoto {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..6 {
            if self.p[i] != other.p[i] {
                return false;
            }
        }

        true
    }
}

impl Eq for TicketNumberLoto {}

impl ToBytes for TicketNumberLoto {
    fn to_bytes(&self) -> Vec<u8> {
        self.p.to_vec()
    }
}

impl ToString for TicketNumberLoto {
    fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.p[0], self.p[1], self.p[2], self.p[3], self.p[4], self.p[5],
        )
    }
}

impl crate::ticket_number::TicketNumber for TicketNumberLoto {
    fn draw_from(tickets: &[Self], source: &[u8]) -> Vec<Self> {
        let hash_value = commit(&bcrypt::hash(source, 12).unwrap());
        let mut rng = ChaCha20Rng::from_seed(hash_value);

        let mut layers = vec![vec![]; 7];
        layers[0] = tickets.to_vec();

        let mut pool = vec![];
        for i in 1..=99u8 {
            pool.push(i);
        }

        let mut picked_numbers = vec![];
        for _ in 0..99 {
            let index = rng.gen_range(0..pool.len());
            let v = pool[index];
            pool.remove(index);
            picked_numbers.push(v);

            for layer_index in (0..=5).rev() {
                let partial_wins = Self::find_parital_win(&layers[layer_index], v);
                let winners = Self::remove_winners(&mut layers[layer_index], partial_wins);
                layers[layer_index + 1].extend(winners);
            }

            if layers[6].len() > 0 {
                println!("{:?}", picked_numbers);
                return layers[6].to_vec();
            }
        }

        vec![]
    }
}
