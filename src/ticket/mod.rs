pub mod draw_period;
pub mod ticket_system;

use chrono::Utc;

use crate::{commitment::ToBytes, customer::Customer, ticket_number::TicketNumber};

pub struct Ticket<T: TicketNumber> {
    pub ticket_id: u64,
    pub draw_period: draw_period::DrawPeriod,
    pub time: chrono::DateTime<Utc>,
    pub customer: Box<dyn Customer>,
    pub predicted_number: T,
}

impl<T: TicketNumber> ToString for Ticket<T> {
    fn to_string(&self) -> String {
        format!(
            "period={}\nid={}\ntime={}\ncustomer={}\nnumber={}",
            self.draw_period.to_string(),
            self.ticket_id,
            self.time,
            self.customer.to_string(),
            self.predicted_number.to_string(),
        )
    }
}

impl<T: TicketNumber> ToBytes for Ticket<T> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.ticket_id.to_be_bytes());
        bytes.extend(self.draw_period.to_bytes());
        bytes.extend(self.time.to_bytes());
        bytes.extend(self.customer.commit());
        bytes.extend(self.predicted_number.to_bytes());

        bytes
    }
}
