use crate::commitment::ToBytes;

pub mod loto;
pub mod number6of55;

pub trait TicketNumber: Clone + Eq + ToBytes + ToString + Sized + Default {
    fn draw_from(tickets: &[Self], source: &[u8]) -> Vec<Self>;
}
