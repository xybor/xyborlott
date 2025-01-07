use chrono::Utc;

use crate::{
    commitment::{commit, Commitment, ToBytes},
    customer::Customer,
    ticket_number::TicketNumber,
};

use super::{draw_period, Ticket};

pub struct TicketSystem<T: TicketNumber> {
    pub draw_period: draw_period::DrawPeriod,
    tickets: Vec<Ticket<T>>,
}

impl<T: TicketNumber> TicketSystem<T> {
    pub fn new(draw_period: draw_period::DrawPeriod) -> Self {
        Self {
            draw_period,
            tickets: vec![],
        }
    }

    pub fn new_ticket<'a, C: Customer + 'static>(
        &'a mut self,
        customer: C,
        predicted_number: T,
    ) -> &'a Ticket<T> {
        let ticket = Ticket {
            ticket_id: self.tickets.len() as u64,
            draw_period: self.draw_period.clone(),
            time: Utc::now(),
            customer: Box::new(customer),
            predicted_number,
        };

        self.tickets.push(ticket);
        self.tickets.last().unwrap()
    }

    pub fn commit(&self) -> Commitment {
        let mut commitment = commit(&self.draw_period);

        for ticket in &self.tickets {
            let mut prev = commitment.to_vec();
            prev.extend(ticket.to_bytes());
            commitment = commit(&prev);
        }

        commitment
    }

    pub fn get_ticket_numbers(&self) -> Vec<T> {
        let mut ticket_numbers = vec![T::default(); self.tickets.len()];
        for i in 0..self.tickets.len() {
            ticket_numbers[i] = self.tickets[i].predicted_number.clone();
        }

        ticket_numbers
    }
}
